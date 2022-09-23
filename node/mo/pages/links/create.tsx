import { Formik } from "formik"
import { GetServerSidePropsContext } from 'next'
import { MOLINKS_CONFIG } from "../../utils/config"
import { useRouter } from "next/router"
import Link from "next/link"
import validator from "validator"
import wordsList from './words-no-swears-data.json'


// Define Prop Interface
interface Props {
    url: string,
    suggestedAlias: string
    redirected: boolean
}

// Define Component
function CreateForm(props: Props) {
    const router = useRouter()
    return (
        <Formik
            initialValues={{ alias: props.suggestedAlias, link: "", isRegex: false }}
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
                    {props.redirected ? 
                    <h2 className="subtitle">No alias <span className='has-text-info'>`{values.alias}`</span> create one?</h2>
                    :
                    <h2 className="subtitle">Create link:</h2>
                    }
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
                    <label className="checkbox">
                        Is Regex redirect: &nbsp;
                        <input className="checkbox" type="checkbox" name="isRegex" value="true" checked={values.isRegex} onChange={handleChange} />
                        </label>
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
export async function getServerSideProps(context: GetServerSidePropsContext) {
    // Get a random unused word but limit to 3 attempts to prevent too many queries.
    let suggestedAlias = context.query.alias || ""
    const redirected = suggestedAlias.length > 0
    if (suggestedAlias.length == 0) {
    for (let i = 0; i < 3; i++) {
        suggestedAlias = wordsList[getRandomInt(0, wordsList.length)]
        const res = await fetch(MOLINKS_CONFIG.API_URL + "/" + suggestedAlias)
        if (res.status != 200) {
            break;
        }
    }
    }

    return {
        props: {
            url: MOLINKS_CONFIG.API_URL,
            suggestedAlias: suggestedAlias,
            redirected: redirected
        },
    }
}

// export component
export default CreateForm