import { GetServerSidePropsContext } from 'next'
import { MoLink } from "../utils/types"
import { openLinksCollection } from "../utils/db"
import { WithId } from "mongodb";


// Define Component
function RedirectPage() {
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
    let docs = await links.aggregate<WithId<MoLink>>([{
      $match: {
        $and:
          [{ isRegex: true },
           { $expr: { $regexFind: { input: query,   regex: "$alias" } } }]
      }
    }]).toArray();
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

export async function getServerSideProps(context: GetServerSidePropsContext) {

  const queryPathParts = context.query.id! as string[];
  const query = queryPathParts.join('/');

  // fetch the MoLink, the param was received via context.query.id
  const url = await findURLForQuery(query);

  return {
    redirect: {
      permanent: false,
      destination: url || "/links/create?alias=" + query,
    }
  };

}

export default RedirectPage
