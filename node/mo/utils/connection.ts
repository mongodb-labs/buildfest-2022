// IMPORT MONGOOSE
import mongoose, { Model, Number } from "mongoose"
import { MOLINKS_CONFIG } from "./config"

// CONNECTING TO MONGOOSE (Get Database Url from .env.local)

// connection function
export const connect = async () => {
    const conn = await mongoose
        .connect(MOLINKS_CONFIG.DATABASE_URL)
        .catch(err => console.log(err))
    console.log("Mongoose Connection Established")

    // OUR MoLink SCHEMA
    const MoLinkSchema = new mongoose.Schema({
        alias: String,
        link: String,
        n: Number,
    })

    // OUR MoLink MODEL
    const MoLink = mongoose.models.MoLink || mongoose.model("MoLink", MoLinkSchema)

    return { conn, MoLink }
}
