import { NextApiResponse } from "next";
import validator from "validator";
import { openLinksCollection } from "../../../utils/db";
import { createMethodHandler } from "../../../utils/handle"
import { encodeExtendedJSON, ExtendedJSONEncoded, MoLink, MoLinkPutJSON } from "../../../utils/types";

export default createMethodHandler({
  async get(_req, resp: NextApiResponse<ExtendedJSONEncoded<MoLink>[]>) {
    const coll = await openLinksCollection();
    const links = await coll.find().sort({ n: -1 }).limit(100).toArray();
    const data = links.map(encodeExtendedJSON);
    resp.json(data);
  },
  async post(req, resp: NextApiResponse<{ error?: string }>) {
    const doc: MoLinkPutJSON = req.body;
    if (!doc.link || !doc.alias) {
      return resp.status(400).json({ error: 'Missing one or more required properties' });
    }
    if (!validator.isURL(doc.link)) {
      return resp.status(400).json({ error: `"${doc.link}" is not a valid URL` });
    }
    const dbDoc = {
      alias: doc.alias,
      link: doc.link,
      isRegex: doc.isRegex || false,
      createdAt: new Date(),
      n: 0,
    }
    const coll = await openLinksCollection();
    await coll.insertOne(dbDoc);
    resp.status(202).json({});
  },
});
