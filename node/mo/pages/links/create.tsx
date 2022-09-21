import { Formik } from "formik"
import { useRouter } from "next/router"
import Link from "next/link"
import validator from "validator"
import { MOLINKS_CONFIG } from "../../utils/config"
import wordsList from './words-no-swears-data.json';


// Define Prop Interface
interface Props {
    url: string,
    randWord: string
}

// Define Component
function CreateForm(props: Props) {
    const router = useRouter()
    return (
        <Formik
            initialValues={{ alias: props.randWord, link: "" }}
            validate={values => {
                const errors: any = {};
                if (!values.alias) {
                    errors.alias = 'Required';
                }
                if (!values.link) {
                    errors.link = 'Required';
                }
                if (!validator.isURL(values.link)) {
                    errors.link = "Invalid URL"
                }

                return errors;
            }}
            onSubmit={async (values, { setSubmitting }) => {
                await fetch(props.url, {
                    method: "post",
                    headers: {
                        "Content-Type": "application/json",
                    },
                    body: JSON.stringify(values),
                }).then(() => {
                    setSubmitting(false);
                    router.push("/")
                })
            }}
        >
            {({
                values,
                errors,
                touched,
                handleChange,
                handleBlur,
                handleSubmit,
                isSubmitting,
                /* and other goodies */
            }) => (
                <form onSubmit={handleSubmit}>
                    <h2 className="subtitle">
                        Edit link:
                    </h2>
                    <div className="field">
                        <label className="label">Alias</label>
                        <div className="control">
                            <input className="input is-info" type="text" name="alias" value={values.alias} onChange={handleChange} />
                            <label className="help is-danger">
                                {errors.alias && touched.alias && errors.alias}
                            </label>
                        </div>
                    </div>
                    <div className="field">
                        <label className="label">Link</label>
                        <div className="control">
                            <input className="input is-link" type="text" name="link" value={values.link} onChange={handleChange} />
                            <label className="help is-danger">
                                {errors.link && touched.link && errors.link}
                            </label>
                        </div>
                    </div>
                    <div className="field is-grouped">
                        <div className="control">
                            <button type="submit" className="button is-primary" disabled={isSubmitting}>Submit</button>
                        </div>
                        <div className="control">
                            <Link href="/"><button className="button is-link is-light">Cancel</button></Link>
                        </div>
                    </div>
                </form>
            )}
        </Formik>
    )
}

function getRandomInt(min: number, max: number) {
    return Math.floor(Math.random() * (max - min + 1)) + min;
}


// export getStaticProps to provide API_URL to component
export async function getServerSideProps(context: any) {
    // Get a random unused word but limit to 3 attempts to prevent too many queries.
    let randWord = ""
    for (let i = 0; i < 3; i++) {
        randWord = wordsList[getRandomInt(0, wordsList.length)]
        const res = await fetch(MOLINKS_CONFIG.API_URL + "/" + randWord)
        if (res.status != 200) {
            break;
        }
    }

    return {
        props: {
            url: MOLINKS_CONFIG.API_URL,
            randWord: randWord
        },
    }
}

// export component
export default CreateForm