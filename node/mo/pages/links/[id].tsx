import { Formik } from "formik"
import { MoLink } from "../../utils/types"
import { useRouter } from "next/router"
import validator from "validator"

// Define Prop Interface
interface Props {
  link: MoLink
  url: string
}

// Define Component
function EditForm(props: Props) {
  const router = useRouter()
  return (
    <Formik
      initialValues={props.link}
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
        await fetch(props.url + "/" + props.link._id, {
          method: "put",
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
              <input className="input is-info" type="text" name="alias" value={values?.alias} onChange={handleChange} />
              <label className="help is-danger">
                {errors.alias && touched.alias && errors.alias}
              </label>
            </div>
          </div>
          <div className="field">
            <label className="label">Link</label>
            <div className="control">
              <input className="input is-link" type="text" name="link" value={values?.link} onChange={handleChange} />
              <label className="help is-danger">
                {errors.link && touched.link && errors.link}
              </label>
            </div>
          </div>
          <button type="submit" className="button is-primary" disabled={isSubmitting}>
            Submit
          </button>
        </form>
      )}
    </Formik>
  )
}

export async function getServerSideProps(context: any) {
  // fetch the MoLink, the param was received via context.query.id
  const res = await fetch(process.env.API_URL + "/" + context.query.id)
  const MoLink = await res.json()

  //return the serverSideProps the MoLink and the url from out env variables for frontend api calls
  return { props: { link: MoLink, url: process.env.API_URL } }
}


// export component
export default EditForm
