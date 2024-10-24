import Image from 'next/image';
import {
  BarChart,
  Bar,
  LineChart,
  Line,
  XAxis,
  YAxis,
  CartesianGrid,
  Tooltip,
  Legend,
  PieChart,
  Pie,
  Cell,
  AreaChart,
  Area,
} from 'recharts';

const viewsData = [
  { date: 'Mar 1', views: 2400, unique: 1398 },
  { date: 'Mar 2', views: 1398, unique: 980 },
  { date: 'Mar 3', views: 9800, unique: 3908 },
  { date: 'Mar 4', views: 3908, unique: 2800 },
  { date: 'Mar 5', views: 4800, unique: 2680 },
  { date: 'Mar 6', views: 3800, unique: 2500 },
  { date: 'Mar 7', views: 4300, unique: 2900 },
];

const categoryData = [
  { name: 'Technology', posts: 45, views: 12500 },
  { name: 'Lifestyle', posts: 30, views: 8900 },
  { name: 'Travel', posts: 25, views: 7600 },
  { name: 'Food', posts: 20, views: 6200 },
];

const recentPosts = [
  {
    id: 1,
    title: 'Getting Started with Next.js 13',
    author: 'John Doe',
    status: 'Published',
    views: 1234,
    publishDate: '2024-03-10',
    thumbnail: '/placeholder.jpg',
  },
  {
    id: 2,
    title: 'The Future of AI in 2024',
    author: 'Jane Smith',
    status: 'Draft',
    views: 0,
    publishDate: '-',
    thumbnail: '/placeholder.jpg',
  },
  {
    id: 3,
    title: 'Ultimate Guide to TypeScript',
    author: 'Mike Johnson',
    status: 'Published',
    views: 892,
    publishDate: '2024-03-08',
    thumbnail: '/placeholder.jpg',
  },
  {
    id: 4,
    title: 'Modern CSS Techniques',
    author: 'Sarah Wilson',
    status: 'Under Review',
    views: 0,
    publishDate: '-',
    thumbnail: '/placeholder.jpg',
  },
];

const recentComments = [
  {
    id: 1,
    user: 'Alex Thompson',
    comment: 'Great article! Very informative.',
    post: 'Getting Started with Next.js 13',
    time: '5 minutes ago',
    avatar: '/avatar1.jpg',
  },
  {
    id: 2,
    user: 'Maria Garcia',
    comment: 'Would love to see more examples on this topic.',
    post: 'Ultimate Guide to TypeScript',
    time: '15 minutes ago',
    avatar: '/avatar2.jpg',
  },
  {
    id: 3,
    user: 'David Chen',
    comment: 'Thanks for sharing these insights!',
    post: 'The Future of AI in 2024',
    time: '1 hour ago',
    avatar: '/avatar3.jpg',
  },
];

const COLORS = ['#0088FE', '#00C49F', '#FFBB28', '#FF8042'];

export default function Home() {
  return (
    <div className="p-6 bg-zinc-950 min-h-screen text-zinc-100">
      {/* Header */}
      <div className="mb-8 flex justify-between items-center">
        <div>
          <h1 className="text-3xl font-bold text-zinc-100">Blog Dashboard</h1>
          <p className="text-zinc-400">
            Manage your content and track performance
          </p>
        </div>
        <button className="bg-zinc-800 text-zinc-100 px-4 py-2 rounded-lg hover:bg-zinc-700 transition-colors">
          Create New Post
        </button>
      </div>

      {/* Stats Cards */}
      <div className="grid grid-cols-1 md:grid-cols-4 gap-6 mb-8">
        {[
          {
            title: 'Total Posts',
            value: '120',
            change: '+4 this week',
            icon: '📝',
          },
          {
            title: 'Total Views',
            value: '45.2K',
            change: '+12% this month',
            icon: '👁️',
          },
          {
            title: 'Comments',
            value: '892',
            change: '+89 this week',
            icon: '💭',
          },
          {
            title: 'Subscribers',
            value: '2.3K',
            change: '+105 this month',
            icon: '📮',
          },
        ].map((stat, index) => (
          <div
            key={index}
            className="bg-zinc-900/50 border border-zinc-800 p-6 rounded-lg"
          >
            <div className="flex items-center justify-between">
              <span className="text-2xl">{stat.icon}</span>
              <span
                className={`text-sm ${stat.change.includes('+') ? 'text-green-500' : 'text-red-500'}`}
              >
                {stat.change}
              </span>
            </div>
            <h3 className="text-zinc-400 text-sm font-medium mt-2">
              {stat.title}
            </h3>
            <p className="text-2xl font-bold text-zinc-100 mt-1">
              {stat.value}
            </p>
          </div>
        ))}
      </div>

      {/* Recent Posts */}
      <div className="bg-zinc-900/50 border border-zinc-800 p-6 rounded-lg mb-8">
        <div className="flex justify-between items-center mb-4">
          <h2 className="text-xl font-semibold text-zinc-100">Recent Posts</h2>
          <button className="text-zinc-400 hover:text-zinc-100">
            View All
          </button>
        </div>
        <div className="overflow-x-auto">
          <table className="min-w-full">
            <thead>
              <tr className="border-b border-zinc-800">
                <th className="px-6 py-3 text-left text-xs font-medium text-zinc-400 uppercase tracking-wider">
                  Post
                </th>
                <th className="px-6 py-3 text-left text-xs font-medium text-zinc-400 uppercase tracking-wider">
                  Author
                </th>
                <th className="px-6 py-3 text-left text-xs font-medium text-zinc-400 uppercase tracking-wider">
                  Status
                </th>
                <th className="px-6 py-3 text-left text-xs font-medium text-zinc-400 uppercase tracking-wider">
                  Views
                </th>
                <th className="px-6 py-3 text-left text-xs font-medium text-zinc-400 uppercase tracking-wider">
                  Published
                </th>
              </tr>
            </thead>
            <tbody className="divide-y divide-zinc-800">
              {recentPosts.map((post) => (
                <tr
                  key={post.id}
                  className="hover:bg-zinc-800/50 transition-colors"
                >
                  <td className="px-6 py-4 whitespace-nowrap">
                    <div className="flex items-center">
                      <div className="h-10 w-10 flex-shrink-0">
                        <Image
                          className="h-10 w-10 rounded-lg object-cover"
                          src={post.thumbnail}
                          alt=""
                          width={40}
                          height={40}
                        />
                      </div>
                      <div className="ml-4">
                        <div className="text-sm font-medium text-zinc-100">
                          {post.title}
                        </div>
                      </div>
                    </div>
                  </td>
                  <td className="px-6 py-4 whitespace-nowrap text-sm text-zinc-400">
                    {post.author}
                  </td>
                  <td className="px-6 py-4 whitespace-nowrap">
                    <span
                      className={`px-2 inline-flex text-xs leading-5 font-semibold rounded-full ${
                        post.status === 'Published'
                          ? 'bg-green-900/50 text-green-400'
                          : post.status === 'Draft'
                            ? 'bg-zinc-800 text-zinc-400'
                            : 'bg-yellow-900/50 text-yellow-400'
                      }`}
                    >
                      {post.status}
                    </span>
                  </td>
                  <td className="px-6 py-4 whitespace-nowrap text-sm text-zinc-400">
                    {post.views}
                  </td>
                  <td className="px-6 py-4 whitespace-nowrap text-sm text-zinc-400">
                    {post.publishDate}
                  </td>
                </tr>
              ))}
            </tbody>
          </table>
        </div>
      </div>

      {/* Recent Comments */}
      <div className="bg-zinc-900/50 border border-zinc-800 p-6 rounded-lg">
        <div className="flex justify-between items-center mb-4">
          <h2 className="text-xl font-semibold text-zinc-100">
            Recent Comments
          </h2>
          <button className="text-zinc-400 hover:text-zinc-100">
            View All
          </button>
        </div>
        <div className="space-y-4">
          {recentComments.map((comment) => (
            <div
              key={comment.id}
              className="flex items-start space-x-4 border-b border-zinc-800 pb-4"
            >
              <Image
                className="h-10 w-10 rounded-full"
                src={comment.avatar}
                alt=""
                width={40}
                height={40}
              />
              <div className="flex-1">
                <div className="flex items-center justify-between">
                  <h3 className="text-sm font-medium text-zinc-100">
                    {comment.user}
                  </h3>
                  <p className="text-sm text-zinc-500">{comment.time}</p>
                </div>
                <p className="text-sm text-zinc-400">on {comment.post}</p>
                <p className="mt-1 text-sm text-zinc-300">{comment.comment}</p>
              </div>
            </div>
          ))}
        </div>
      </div>
    </div>
  );
}
