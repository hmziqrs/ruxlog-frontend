import {
  Home,
  Users,
  BookOpen,
  MessageSquare,
  FolderTree,
  Tags,
} from 'lucide-react';
import Link from 'next/link';

import {
  Sidebar,
  SidebarContent,
  SidebarGroup,
  SidebarGroupContent,
  SidebarGroupLabel,
  SidebarMenu,
  SidebarMenuButton,
  SidebarMenuItem,
  useSidebar,
} from '@/components/ui/sidebar';

const items = [
  {
    title: 'Home',
    icon: Home,
    href: '/',
  },
  {
    title: 'Users',
    icon: Users,
    href: '/users',
  },
  {
    title: 'Blogs',
    icon: BookOpen,
    href: '/blogs',
  },
  {
    title: 'Comments',
    icon: MessageSquare,
    href: '/comments',
  },
  {
    title: 'Categories',
    icon: FolderTree,
    href: '/categories',
  },
  {
    title: 'Tags',
    icon: Tags,
    href: '/tags',
  },
];

export function AppSidebar() {
  const { open } = useSidebar();
  // <Sidebar className="border-r" collapsible="icon">
  return (
    <Sidebar>
      <SidebarContent>
        <SidebarGroup>
          <SidebarGroupLabel>Dashboard</SidebarGroupLabel>
          <SidebarGroupContent>
            <SidebarMenu>
              {items.map((item) => (
                <SidebarMenuItem key={item.title}>
                  <SidebarMenuButton asChild>
                    <Link href={item.href}>
                      <item.icon className="h-4 w-4" />
                      <span>{item.title}</span>
                    </Link>
                  </SidebarMenuButton>
                </SidebarMenuItem>
              ))}
            </SidebarMenu>
          </SidebarGroupContent>
        </SidebarGroup>
      </SidebarContent>
    </Sidebar>
  );
}
