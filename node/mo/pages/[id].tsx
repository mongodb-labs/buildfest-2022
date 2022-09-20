import { useRouter } from "next/router"
import { MOLINKS_CONFIG } from "../utils/config"
import {connect} from "../utils/connection";

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
  const { MoLink } = await connect() // connfindOneAndUpdateect to database
  const molink = await MoLink.findOneAndUpdate({alias: context.query.id}, {$inc: {n: 1}}).exec()
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
