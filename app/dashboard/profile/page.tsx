import { SearchParams } from 'nuqs/parsers';
import ProfileViewPage from './_components/profile-view-page';

type pageProps = {
  searchParams: SearchParams;
};

export const metadata = {
  title: 'Dashboard : Profile'
};

export default async function Page(props: pageProps) {
  const searchParams = await props.searchParams;
  return <ProfileViewPage />;
}
