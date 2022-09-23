import { NextApiRequest, NextApiResponse } from "next"
import { openLinksCollection } from "../../../utils/db"
import { createMethodHandler } from "../../../utils/handle"
import { MoLinkPutJSON } from "../../../utils/types"
import { WithId } from "mongodb";

const handler = async (req: NextApiRequest, res: NextApiResponse) => {
    //function for catch errors
    const catcher = (error: Error) => res.status(400).json({ error })

    // GRAB ID FROM req.query (where next stores params)
    const id: string = decodeURIComponent(req.query.id as string)
    const links = await openLinksCollection();
    // Potential Responses for /links/:id
    const handler = createMethodHandler({
        // RESPONSE FOR GET REQUESTS
        async get(req: NextApiRequest, res: NextApiResponse) {
            var found = await links.findOne({ alias: id });
            if (found == null) {
                res.status(404).json({})
            } else {
                res.json(found)
            }
        },
        // RESPONSE PUT REQUESTS
        async put(req: NextApiRequest, res: NextApiResponse) {
            const data: Partial<WithId<MoLinkPutJSON>> = req.body;
            delete data._id;
            res.json(
                await links.findOneAndUpdate({ alias: id }, { $set: data }, { upsert: true })
            )
        },
        // RESPONSE FOR DELETE REQUESTS
        async delete(req: NextApiRequest, res: NextApiResponse) {
            res.json(await links.deleteOne({ alias: id }));
        },
    });

    return handler(req, res);
}

export default handler;
