import { useRouter } from "next/router"
import { connect } from "../utils/connection";

// Define Prop Interface
interface Props {
  url: string
}

// Define Component
function RedirectPage(props: Props) {
  const router = useRouter()
  console.log(props)
  // router.push(props.url)
}

export async function getServerSideProps(context: any) {
  let url: string = '/'

  const query = context.query.id.join("/")
  
  // fetch the MoLink, the param was received via context.query.id
  const { MoLink } = await connect() // connect to database
  const molink = await MoLink.findOneAndUpdate({ alias: query }, { $inc: { n: 1 } }).exec()
  if (molink) {
    url = molink.link
  } else {
    // Search for regexs and replace :)
    let docs = await MoLink.aggregate([{ $match: { $expr: { $regexFind: { input: query, regex: "$alias" } } } }]).exec()
    if (docs.length > 0) {
      let link = docs[0]
      await MoLink.updateOne({ _id: link._id }, { $inc: { n: 1 } }).exec()
      url = query.replace(new RegExp(link.alias), link.link)
    }
  }

  return {
    redirect: {
      permanent: false,
      destination: url,
    },
    props: { url: url },
  };

}

export default RedirectPage
