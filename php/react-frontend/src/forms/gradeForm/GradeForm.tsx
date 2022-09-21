import { Fragment, useState } from 'react';
import { JsonForms } from '@jsonforms/react';
import schema from './schema.json';
import uischema from './uischema.json';
import {
  materialCells,
  materialRenderers,
} from '@jsonforms/material-renderers';
import { submit } from '../submit';

const initialData = {};
const GradeForm = () => {
  const [data, setData] = useState<any>(initialData);
  return (
    <Fragment>
      <h1>Grade An Assignment:</h1>
      <div className='demoForm'>
        <JsonForms
          schema={schema}
          uischema={uischema}
          data={data}
          renderers={materialRenderers}
          cells={materialCells}
          onChange={({ errors, data }) => setData(data)}
        />
      </div>
      <button
        onClick={() => {
          submit('/grades', data);
        }}
      >
        Submit
      </button>
    </Fragment>
  );
};

export default GradeForm;
