# Admin Users Store Documentation

## Overview

The admin users store manages user data with full CRUD operations, pagination support, and state management. It interfaces with the backend admin API endpoints for user management.

## Backend API Endpoints

The store integrates with the following backend routes:

- `POST /admin/list` - Fetch paginated list of users
- `GET /admin/view/{user_id}` - Get details of a specific user
- `POST /admin/create` - Create a new user
- `POST /admin/update/{user_id}` - Update an existing user
- `POST /admin/delete/{user_id}` - Delete a user

All endpoints require admin permission middleware and verified user status.

## Data Structures

### User

```rust
pub struct User {
    pub avatar: Option<String>,
    pub created_at: String,
    pub email: String,
    pub id: i32,
    pub is_verified: bool,
    pub name: String,
    pub role: UserRole,
    pub two_fa_backup_codes: Option<String>,
    pub two_fa_enabled: bool,
    pub two_fa_secret: Option<String>,
    pub updated_at: String,
}
```

### UsersAddPayload

```rust
pub struct UsersAddPayload {
    pub avatar: Option<String>,
    pub email: String,
    pub is_verified: bool,
    pub name: String,
    pub role: UserRole,
}
```

### UsersEditPayload

```rust
pub struct UsersEditPayload {
    pub avatar: Option<String>,
    pub email: Option<String>,
    pub is_verified: Option<bool>,
    pub name: Option<String>,
    pub role: Option<UserRole>,
}
```

## State Structure

```rust
pub struct UsersState {
    pub add: GlobalSignal<StateFrame<()>>,
    pub edit: GlobalSignal<HashMap<i32, StateFrame<()>>>,
    pub remove: GlobalSignal<HashMap<i32, StateFrame<()>>>,
    pub list: GlobalSignal<StateFrame<PaginatedList<User>>>,
    pub view: GlobalSignal<HashMap<i32, StateFrame<Option<User>>>>,
}
```

## Pagination Support

The list endpoint returns a paginated response:

```json
{
  "data": [ /* array of User objects */ ],
  "page": 1,
  "per_page": 20,
  "total": 50
}
```

The `PaginatedList<User>` structure provides:
- `data: Vec<User>` - The user records for the current page
- `page: u64` - Current page number
- `per_page: u64` - Number of items per page
- `total: u64` - Total number of users
- `has_next_page()` - Check if there's a next page
- `has_previous_page()` - Check if there's a previous page

## Usage

### Access the Store

```rust
use crate::store::users::use_user;

let users_state = use_user();
```

### List Users

```rust
// Fetch the paginated list
spawn(async move {
    users_state.list().await;
});

// Access the list data
let list = users_state.list.read();
if let Some(paginated) = &list.data {
    for user in &paginated.data {
        println!("{}: {}", user.id, user.name);
    }
    println!("Page {} of {}", paginated.page, paginated.total / paginated.per_page);
}
```

### View User Details

```rust
let user_id = 42;
spawn(async move {
    users_state.view(user_id).await;
});

// Access the viewed user
let view_map = users_state.view.read();
if let Some(state) = view_map.get(&user_id) {
    if let Some(Some(user)) = &state.data {
        println!("User: {}", user.name);
    }
}
```

### Create User

```rust
let payload = UsersAddPayload {
    avatar: None,
    email: "user@example.com".to_string(),
    is_verified: true,
    name: "John Doe".to_string(),
    role: UserRole::User,
};

spawn(async move {
    users_state.add(payload).await;
});

// Check creation status
let add_state = users_state.add.read();
if add_state.is_success() {
    println!("User created successfully!");
}
```

### Update User

```rust
let user_id = 42;
let payload = UsersEditPayload {
    name: Some("Jane Doe".to_string()),
    role: Some(UserRole::Admin),
    ..Default::default()
};

spawn(async move {
    users_state.edit(user_id, payload).await;
});

// Check update status
let edit_map = users_state.edit.read();
if let Some(state) = edit_map.get(&user_id) {
    if state.is_success() {
        println!("User updated successfully!");
    }
}
```

### Delete User

```rust
let user_id = 42;
spawn(async move {
    users_state.remove(user_id).await;
});

// Check deletion status
let remove_map = users_state.remove.read();
if let Some(state) = remove_map.get(&user_id) {
    if state.is_success() {
        println!("User deleted successfully!");
    }
}
```

### Reset State

```rust
users_state.reset();
```

## State Management Features

### Automatic List Updates

The store automatically updates the list when operations succeed:

- **After create**: New user is prepended to the list, total count incremented
- **After update**: User record is updated in the list and view cache
- **After delete**: User is removed from the list, total count decremented

### Per-Item State Tracking

Edit, remove, and view operations maintain state per user ID using `HashMap`, allowing:
- Multiple simultaneous operations on different users
- Independent loading/error states for each user
- Efficient state lookup by user ID

### State Frame Status

Each operation tracks its state through `StateFrame`:

- `Init` - Initial state, no operation started
- `Loading` - Operation in progress
- `Success` - Operation completed successfully
- `Failed` - Operation failed with error message

Check states using:
```rust
state.is_init()
state.is_loading()
state.is_success()
state.is_failed()
```

## Error Handling

All API errors are captured and stored in the state frame:

```rust
let list_state = users_state.list.read();
if list_state.is_failed() {
    if let Some(error_msg) = &list_state.message {
        eprintln!("Error loading users: {}", error_msg);
    }
}
```

## Best Practices

1. **Always check loading state** before showing data to users
2. **Handle all three states** (init, loading, success/failed) in UI components
3. **Use spawn for async operations** to avoid blocking the UI
4. **Reset state when navigating away** from user management screens
5. **Check individual operation states** when performing batch operations
6. **Update UI reactively** using Dioxus signals - the store will trigger re-renders automatically

## Integration with UI Components

The store integrates seamlessly with the `DataTableScreen` component for displaying paginated user lists:

```rust
use crate::components::DataTableScreen;
use crate::store::users::use_user;

#[component]
pub fn UsersList() -> Element {
    let users_state = use_user();
    let list = users_state.list.read();
    
    rsx! {
        DataTableScreen::<User> {
            page_title: "Users",
            list: list.clone(),
            show_pagination: true,
            on_prev: move |_| {
                // Handle previous page
            },
            on_next: move |_| {
                // Handle next page
            },
            // ... other props
        }
    }
}
```
