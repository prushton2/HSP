import './activities.css'
import { Http } from '../../axios/axios'
import type { ApiResponseObjects, Tables } from '../../axios/structs'
import { formatProperly } from '../../components/Format'
import { prompt } from '../../components/Modal'
import { useEffect, useState, type JSX } from 'react'
import { Toast } from '../../components/toast'

export default function Activities(): JSX.Element {
    const [activities, setActivities] = useState<Tables.Activity[]>([])
    const [selectedActivity, setSelectedActivity] = useState<string>("")

    const [_, setInfo] = useState<Tables.Activity>({uuid: "", name: "", staff: [], dates: []} as Tables.Activity)
    const [absentStudents, setAbsentStudents] = useState<ApiResponseObjects.FullStudent[]>([])
    const [presentStudents, setPresentStudents] = useState<ApiResponseObjects.FullStudent[]>([])
    
    useEffect(() => {
        async function init() {
            let date = new Date();
            let today = 1000 * ((date.getTime()/1000) - (date.getTime()/1000)%86400 + 3600*4)
            let result = await Toast.WrapErr<Tables.Activity[], string>(() => Http.Activity.Search(today))

            if(result.is_ok()) {
                setActivities(result.into_ok())
            }
        }
        init()
    }, [])

    useEffect(() => {
        if(selectedActivity == "") {
            setAbsentStudents([])
            setPresentStudents([])
            setInfo({uuid: "", name: "", staff: [], dates: []} as Tables.Activity)
            return
        }
        async function init() {
            let result = await Toast.WrapErr<[Tables.Activity, ApiResponseObjects.FullStudent[]], string>(() => Http.Activity.Get(selectedActivity))
            if(result.is_ok()) {
                setInfo(result.into_ok()[0])
                setAbsentStudents(result.into_ok()[1])
                setPresentStudents([])
            }
        }
        init()
    }, [selectedActivity])

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
                            setPresentStudents([e, ...presentStudents].sort((a, b) => a.number - b.number))
                            setAbsentStudents(absentStudents.filter((_, j) => {return i != j}))
                        } else {
                            setAbsentStudents([e, ...absentStudents].sort((a, b) => a.number - b.number))
                            setPresentStudents(presentStudents.filter((_, j) => {return i != j}))
                        }
                    }}> {complete ? "Absent" : "Present"} </button>
    
                </div>
            })
        }

    return (
        <div className="activitiespage">
            <div className="attendees">
                {absentStudents.length > 0 ? <h2>Absent Students</h2> : <></>}
                {RenderStudents(absentStudents, false)}
                {presentStudents.length > 0 ? <h2>Present Students</h2> : <></>}
                {RenderStudents(presentStudents, true)}
            </div>

            <select className='activity-select' onChange={(e) => setSelectedActivity(e.target.value)}> 
                <option value="">No Activity</option>
                {RenderActivityButtons(activities)} 
            </select>
        </div>
    )
}

function RenderActivityButtons(activities: Tables.Activity[]): JSX.Element[] {
    return activities.map((e) => {
        return <option value={e.uuid}>{e.name}</option>
    })
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