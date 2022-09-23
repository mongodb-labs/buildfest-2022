import { decodedExtendedJSON, ExtendedJSONEncoded, MoLink } from "../utils/types"
import Link from "next/link"
import { MOLINKS_CONFIG } from "../utils/config"
import { useEffect, useState } from "react";
import { WithId } from 'mongodb';


// Define the components props

interface IndexProps {
  links: ExtendedJSONEncoded<WithId<MoLink>>[];
}

function deleteAlias(alias: string) {
  fetch('/api/links/' + alias,  {
    method: 'DELETE'
  })
  window.location.reload();
}

function rowFromData(item: ExtendedJSONEncoded<WithId<MoLink>>) {
  const link = decodedExtendedJSON(item);

  const dateFormat = new Intl.DateTimeFormat(undefined, {
    dateStyle: 'medium',
    timeStyle: 'medium',
  });

  const [dateString, setDateString] = useState('');
  useEffect(() => setDateString(
    link.createdAt
      ? dateFormat.format(new Date(link.createdAt))
      : '[no date recorded]'
  ));
  const editPath = encodeURIComponent(link.alias);
  return (
    <tr key={link._id.toString()}>
      <td>{link.alias}</td>
      <td><a href={link.link}>{link.link}</a></td>
      <td>{link.n || 0}</td>
      <td>{dateString}</td>
      <td><Link href={`./links/${editPath}`}><button className="button is-link is-primary">Edit</button></Link></td>
      <td><button className="button" onClick={ () => deleteAlias(link.alias) }>Delete</button></td>
    </tr>
  );
}

// define the page component
function Index(props: IndexProps) {
  const { links } = props

  return (
    <div className="container layout vbox" style={{ gap: '30px' }}>
      <Link href="./links/create"><button className="button is-primary">Create a New Link</button></Link>
      <div className="popular-box layout vbox" style={{
        border: '1px solid #0004',
        boxShadow: '0 4px 15px #0005, 0 2px 5px #0006'
      }}>
        <h2 className="layout self-center" style={{ fontSize: '15pt', fontWeight: 'bold', fontStyle: 'italic', margin: '5px' }}>Most Popular:</h2>
        <div className="container">
          <table className="table">
            <thead>
              <tr>
                <th>Alias</th>
                <th>Link</th>
                <th>Count</th>
                <th>Created</th>
                <th></th>
              </tr>
            </thead>
            <tbody>
              {links.map(rowFromData)}
            </tbody>
          </table>
        </div>
      </div>
    </div>
  );
}

// GET PROPS FOR SERVER SIDE RENDERING
export async function getServerSideProps(): Promise<{ props: IndexProps }> {
  // get molink data from API
  const res = await fetch(MOLINKS_CONFIG.API_URL)
  const links: ExtendedJSONEncoded<WithId<MoLink>>[] = await res.json();

  // return props
  return { props: { links } };
}

export default Index
