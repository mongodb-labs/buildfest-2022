import { AppContext } from "next/app"
import { useRouter } from "next/router"
import { openLinksCollection } from "../utils/db"
import { MoLink } from "../utils/types"
import { WithId } from "mongodb";

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

async function findURLForQuery(query: string): Promise<string | null> {
  const links = await openLinksCollection();
  const found = await links.findOneAndUpdate({ alias: query }, { $inc: { n: 1 } });
  let link = found.value;
  let url: string | null = null;
  if (link) {
    url = link.link;
  } else {
    // Search for regexs and replace :)
    let docs = await links.aggregate<WithId<MoLink>>([{ $match: { $expr: { $regexFind: { input: query, regex: "$alias" } } } }]).toArray();
    if (docs.length > 0) {
      link = docs[0];
      await links.updateOne({ _id: link._id }, { $inc: { n: 1 } });
      url = query.replace(new RegExp(link.alias), link.link)
    } else {
      console.error(`Request for non-existent link "${query}"`);
    }
  }
  return url;
}

export async function getServerSideProps(context: AppContext) {
  const queryPathParts = context.router.query.id! as string[];
  const query = queryPathParts.join('/');

  // fetch the MoLink, the param was received via context.query.id
  const url = await findURLForQuery(query);

  return {
    redirect: {
      permanent: false,
      destination: url,
    },
    props: { url: url },
  };

}

export default RedirectPage
