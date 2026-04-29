import './search.css'
import { Http } from '../../axios/axios'
import type { ApiRequestObjects, ApiResponseObjects } from '../../axios/structs'
import { formatProperly } from '../../components/Format'
import { Modal, prompt } from '../../components/Modal'
import { useState, type JSX } from 'react'

export default function Search(): JSX.Element {
    const [searchState, setSearchState] = useState({fname: null, lname: null, room: null, hall: null} as ApiRequestObjects.SearchStudent)
    const [results, setResults] = useState([] as ApiResponseObjects.FullStudent[])

    let search_options = [
        <option value="Unspecified">Unspecified</option>,
        <option value="alumni">Alumni</option>,
        <option value="stone">Stone</option>,
        <option value="davis">Davis</option>,
    ]
    return (
        <>
            {RenderSearchResults(results)}

            <table className='searchtable'>
            <tbody>
                <tr>
                    <td className='lefttd'>First Name</td> <td><input onChange={(e) => setSearchState({...searchState, fname: e.target.value == "" ? null : e.target.value})}/> </td>
                </tr>
                <tr>
                    <td className='lefttd'>Last Name</td> <td><input onChange={(e) => setSearchState({...searchState, lname: e.target.value == "" ? null : e.target.value})}/></td>
                </tr>
                <tr>
                    <td className='lefttd'>Room</td> <td><input type="number" defaultValue={0} onChange={(e) => setSearchState({...searchState, room: e.target.valueAsNumber == 0 ? null : e.target.valueAsNumber})}/></td>
                </tr>
                <tr>
                    <td className='lefttd'>Hall</td> <td><select  onChange={(e) => setSearchState({...searchState, hall: e.target.value == "Unspecified" ? null : e.target.value})}> {search_options} </select></td>
                </tr>
                <tr>
                    <td /><td><button onClick={async () => setResults(await Http.Student.Search(searchState))}> Search </button></td>
                </tr>
            </tbody>
            </table>
        </>
    )
}

function RenderSearchResults(students: ApiResponseObjects.FullStudent[]): JSX.Element[] {
    let html: JSX.Element[] = [];
    
    students.forEach(e => {
        html.push(<>
            <button className="searchresult" onClick={async() => {await prompt.show("", <StudentModal student={e} />)}}>{e.fname} {e.lname}</button><br />
        </>)
    })
    
    return html;
}

function StudentModal({student}: {student: ApiResponseObjects.FullStudent}): JSX.Element {
    return <div className="studentModal">
        <label>{student.fname} {student.lname}</label> <br />
        <label>{student.pronouns}</label> <br />
        <label>Student #{student.number}</label> <br />
        <label>{formatProperly(student.hall)} Hall, Room {student.room}</label> <br />
        <label>{formatProperly(student.wing)} Wing</label>
    </div>
}