import './nursing.css'
import { Http } from '../../axios/axios';
import type { ApiResponseObjects } from '../../axios/structs';
import { formatProperly } from '../../components/Format';
import { prompt } from '../../components/Modal';
import { Toast } from '../../components/toast';
import { useEffect, useRef, useState, type JSX } from 'react'

export default function Nursing(): JSX.Element {
    const [numbers, setNumbers] = useState<number[]>([]);
    const [loading, setLoading] = useState<boolean>(false);
    const [incompleteStudents, setIncompleteStudents] = useState<ApiResponseObjects.FullStudent[]>([])
    const [completeStudents, setCompleteStudents] = useState<ApiResponseObjects.FullStudent[]>([])

    let ref = useRef(0);

    useEffect(() => {
        if(numbers.length == 0) {
            setIncompleteStudents([]);
            setCompleteStudents([]);
            return
        }

        clearTimeout(ref.current);
        setLoading(true);

        ref.current = setTimeout(async () => {
            let response = await Toast.WrapErr(() => Http.Student.Numbers(numbers));
            if(response.is_ok()) {
                console.log(response.into_ok())
                setIncompleteStudents(response.into_ok().sort((a, b) => a.number - b.number))
                setCompleteStudents([])
                setLoading(false);
            }
        }, 1000)

    }, [numbers])

    function RenderStudents(students: ApiResponseObjects.FullStudent[], complete: boolean): JSX.Element[] {
        if(students.length == 0) {
            return [];
        }
        
        return students.map<JSX.Element>((e, i) => {
            return <div className={complete ? "completeStudent" : "incompleteStudent"}>

                <button className="studentButton" onClick={async() => {
                    await prompt.show("", <StudentModal student={e} />)}}
                >{e.fname} {e.lname}</button>

                <button className='numberLabel'>
                    #{e.number}
                </button>

                <button className='removeButton' onClick={() => {
                    if(!complete) {
                        setCompleteStudents([e, ...completeStudents].sort((a, b) => a.number - b.number))
                        setIncompleteStudents(incompleteStudents.filter((_, j) => {return i != j}))
                    } else {
                        setIncompleteStudents([e, ...incompleteStudents].sort((a, b) => a.number - b.number))
                        setCompleteStudents(completeStudents.filter((_, j) => {return i != j}))
                    }
                }}> {complete ? "Unsend" : "Send"} </button>

            </div>
        })
    }

    return (
        <div className="nn-container">
            <div className="students">
                {loading ? <h2>Loading...</h2> : <>
                    {incompleteStudents.length > 0 ? <h2>Students</h2> : <></>}
                    {RenderStudents(incompleteStudents, false)}
                    {completeStudents.length > 0 ? <h2>Sent Students</h2> : <></>}
                    {RenderStudents(completeStudents, true)}
                </>}
            </div>

            <table className='nntable'>
            <tbody>
                <tr>
                    <td className='lefttd'>Enter Nursing Numbers</td>
                    <td>
                        <input onChange={(e) => {
                            setNumbers(e.target.value.split(/[^0-9]/).filter((v) => v != "").map((v) => parseInt(v)))
                        }}/>
                    </td>
                </tr>
            </tbody>
            </table>
        </div>
    )
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