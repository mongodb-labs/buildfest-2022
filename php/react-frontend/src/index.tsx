import ReactDOM from 'react-dom';
import CourseForm from './forms/courseForm/CourseForm';
import TeacherForm from './forms/teacherForm/TeacherForm';
import StudentForm from './forms/studentForm/StudentForm';
import GradeForm from './forms/gradeForm/GradeForm';
import { BrowserRouter as Router, Routes, Route, Link } from 'react-router-dom';
import { FetchAndDisplay } from './getVals';

const gradeDisplayProp = { path: '/grades' };
const studentDisplayProp = { path: '/students' };
const teacherDisplayProp = { path: '/teachers' };
const courseDisplayProp = { path: '/courses' };

const AppRouter = (
  <Router>
    <div>
      <nav>
        <ul>
          <li>
            <Link to='/'>Home</Link>
          </li>
          <li>
            <Link to='/students/form'>Students Form</Link>
          </li>
          <li>
            <Link to='/students'>Students</Link>
          </li>
          <li>
            <Link to='/teachers/form'>Teachers Form</Link>
          </li>
          <li>
            <Link to='/teachers'>Teachers</Link>
          </li>
          <li>
            <Link to='/grades/form'>Grades Form</Link>
          </li>
          <li>
            <Link to='/grades'>Grades</Link>
          </li>
          <li>
            <Link to='/courses/form'>Courses Form</Link>
          </li>
          <li>
            <Link to='/courses'>Courses</Link>
          </li>
        </ul>
      </nav>
      <Routes>
        <Route path='/students/form' element={<StudentForm />} />
        <Route
          path='/students'
          element={<FetchAndDisplay {...studentDisplayProp} />}
        />
        <Route path='/teachers/form' element={<TeacherForm />} />
        <Route
          path='/teachers'
          element={<FetchAndDisplay {...teacherDisplayProp} />}
        />
        <Route path='/grades/form' element={<GradeForm />} />
        <Route
          path='/grades'
          element={<FetchAndDisplay {...gradeDisplayProp} />}
        />
        <Route path='/courses/form' element={<CourseForm />} />
        <Route
          path='/courses'
          element={<FetchAndDisplay {...courseDisplayProp} />}
        />
        <Route path='/' element={<p>Home</p>} />
      </Routes>
    </div>
  </Router>
);

ReactDOM.render(AppRouter, document.getElementById('root'));
