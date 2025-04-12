Here's a detailed documentation and README for the Huxlog Admin project:

# Huxlog Admin Dashboard

Huxlog Admin is a modern, responsive admin dashboard built with Next.js 13+ and TypeScript. It provides a comprehensive interface for managing blog content, users, categories, and tags.

## 🚀 Features

- **Modern Tech Stack**: Built with Next.js 15, TypeScript, and Tailwind CSS
- **Authentication System**: Secure login with role-based access control
- **Content Management**:
  - Posts: Create, edit, delete, and manage blog posts with rich text editing
  - Categories: Organize content with hierarchical categories
  - Tags: Flexible content tagging system
  - Users: User management with role-based permissions
- **Real-time Data**: Live updates and state management with Zustand
- **Rich Text Editor**: Advanced MDX editor for content creation
- **Responsive Design**: Mobile-first approach with adaptive layouts
- **Dark Mode**: Built-in dark mode support
- **Analytics Dashboard**: Visual data representation with charts

## 📦 Project Structure

```
huxlog-nextjs/admin/
├── src/
│   ├── app/                    # Next.js 15 app directory
│   │   ├── (dashboard)/       # Dashboard routes
│   │   ├── auth/             # Authentication routes
│   │   └── layout.tsx        # Root layout
│   ├── components/           # Reusable components
│   │   ├── ui/              # UI components (shadcn/ui)
│   │   └── [feature]/       # Feature-specific components
│   ├── store/               # State management
│   │   ├── auth/           # Authentication store
│   │   ├── post/           # Posts store
│   │   ├── category/       # Categories store
│   │   ├── tag/           # Tags store
│   │   └── user/          # Users store
│   ├── hooks/             # Custom React hooks
│   ├── lib/              # Utility functions
│   └── services/         # API services
```

## 🛠️ Technical Architecture

### State Management

The project uses Zustand with Immer for state management, providing:

- Type-safe state updates
- Immutable state handling
- Modular store structure
- Easy integration with React components

### API Integration

- Axios-based API client with:
  - Automatic camelCase/snake_case conversion
  - Request/response interceptors
  - Error handling
  - CSRF protection

### Component Structure

Components follow a "Smart and Dumb" pattern:

- Smart components (containers) handle logic and state
- Dumb components (UI) handle presentation
- Brain files separate business logic

### Forms and Validation

- React Hook Form for form handling
- Zod for schema validation
- Custom form hooks for reusability

## 🚀 Getting Started

### Prerequisites

- Node.js 18+
- Yarn or npm
- Git

### Installation

1. Clone the repository:

```bash
git clone https://github.com/yourusername/huxlog-admin.git
cd huxlog-admin
```

2. Install dependencies:

```bash
yarn install
```

3. Create a `.env.local` file:

```env
NEXT_PUBLIC_API=http://localhost:8000
NEXT_PUBLIC_CSRF_TOKEN=your_csrf_token
```

4. Start the development server:

```bash
yarn dev
```

## 📝 Configuration

### Environment Variables

- `NEXT_PUBLIC_API`: API base URL
- `NEXT_PUBLIC_CSRF_TOKEN`: CSRF token for API requests

### Tailwind Configuration

The project uses a custom Tailwind configuration with:

- Custom color scheme
- Dark mode support
- Typography plugin
- Animation utilities

## 🔒 Authentication

The authentication system includes:

- JWT-based authentication
- Role-based access control
- Protected routes
- Automatic token refresh

### Roles

- `super-admin`: Full system access
- `admin`: Administrative access
- `moderator`: Content moderation
- `author`: Content creation
- `user`: Basic access

## 📋 Features Detail

### Dashboard

- Overview statistics
- Recent activity
- Performance metrics
- Visual data representation

### Post Management

- Rich text editor with MDX support
- Image upload
- Draft/publish status
- Category and tag assignment
- SEO fields

### Category Management

- Hierarchical categories
- Cover images
- Description and metadata
- Parent/child relationships

### Tag Management

- Tag creation and editing
- Tag assignments
- Usage statistics

### User Management

- User creation
- Role assignment
- Account verification
- Profile management

## 📄 License

This project is licensed under the MIT License.

## 🤝 Support

For support, email support@example.com or join our Slack channel.
