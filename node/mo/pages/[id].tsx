import { useRouter } from "next/router"

// Define Prop Interface
interface Props {
  url: string
}

// Define Component
function RedirectPage(props: Props) {
  const router = useRouter()
  router.push(props.url)
}

export async function getServerSideProps(context: any) {
  // fetch the MoLink, the param was received via context.query.id
  const res = await fetch(process.env.API_URL + "/" + context.query.id)
  const molink = await res.json()
  let url = '/'
  if (molink) {
    url = molink.link
  }
  return {
    redirect: {
      permanent: false,
      destination: url,
    },
    props: {url: url},
  };
}

export default RedirectPage
