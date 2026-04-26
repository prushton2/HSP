import { useEffect, useState, type JSX } from 'react'
import './Admin.css'
import { GetAllStudentInfo, HttpCreateStudent, HttpCreateUser, HttpDeleteStudent, HttpEditStudent, HttpGetStudent, HttpUpdateUser } from '../../axios/axios'
import { DefaultAllStudentInfo, type EditStudent, type FullStudentInfo, type TableUsers, type StudentTablesResponse, type UpdateUser } from '../../axios/structs';
import { Modal, prompt } from '../../components/Modal';
import HoverDropdown from '../../components/HoverDropdown';
import RenderTable from './RenderTable';

function Admin() {
    const [studentInfo, setStudentInfo] = useState<StudentTablesResponse>({} as StudentTablesResponse);
    const [selectedUUID, setSelectedUUID] = useState<string>("");

    useEffect(() => {
        async function init() {
            let info = await GetAllStudentInfo()
            setStudentInfo(info);
        }
        init();
    }, [])

    return (
    <>
        <Modal />
        <div className="title">
        <h1>HSP Admin</h1>
        </div>
        <div className='ribbon'>
            <HoverDropdown title="Student" buttons={[
                ["Create",  async () => {await prompt.show("Create Student", <CreateStudent/>)}],
                ["Edit",    async () => {await prompt.show("Edit Student", <EditStudent init_uuid={selectedUUID == "0" ? "" : selectedUUID}/>)}],
                ["Get",     async () => {await prompt.show("Get Student", <GetStudent init_uuid={selectedUUID == "0" ? "" : selectedUUID}/>)}],
                ["Delete",  async () => {await prompt.show("Delete Student", <DeleteStudent init_uuid={selectedUUID == "0" ? "" : selectedUUID}/>)}],
            ]}/>

            <HoverDropdown title="Activities" buttons={[
                ["Create", () => {}],
                ["Clone", () => {}],
                ["Assign", () => {}],
                ["Edit", () => {}]
            ]}/>

            <HoverDropdown title="Access" buttons={[
                ["Grant",  async () => {await prompt.show("Grant Access", <GrantAccess />)}],
                ["Update", async () => {await prompt.show("Update Access", <EditUser init_uuid={selectedUUID == "0" ? "" : selectedUUID}/>)}],
                ["Revoke", async () => {}],
            ]}/>
        </div>
        <div className="tables">
            <RenderTable select={(u) => {setSelectedUUID(u)}} selected={selectedUUID} info={studentInfo.studentinfo} tag="student_info" />
            <RenderTable select={(u) => {setSelectedUUID(u)}} selected={selectedUUID} info={studentInfo.residence} tag="residencies" />
            <RenderTable select={(u) => {setSelectedUUID(u)}} selected={selectedUUID} info={studentInfo.users} tag="users" />
            <RenderTable select={(u) => {setSelectedUUID(u)}} selected={selectedUUID} info={studentInfo.tokens} tag="tokens" />
            {/* <RenderTable select={(u) => {setSelectedUUID(u)}} selected={selectedUUID} info={studentInfo.student_activities} tag="student_activities" />
            <RenderTable select={(u) => {setSelectedUUID(u)}} selected={selectedUUID} info={studentInfo.activities} tag="activities" /> */}
        </div>
    </>
    )
}

export default Admin

function EditStudent({init_uuid}: {init_uuid: string}): JSX.Element {
    const [editData, setEditData] = useState({} as EditStudent)
    const [uuid, setUuid] = useState(init_uuid)
    const [field, setField] = useState("")
    const [value, setValue] = useState("")

    useEffect(() => {
        let int_data = field == "number" || field == "room" ? parseInt(value) : -1;
        
        setEditData({
            uuid: uuid,
            field: field,
            int_field: int_data,
            str_field: int_data == -1 ? value : ""
        } as EditStudent);

    }, [uuid, field, value])

    function Options(): JSX.Element {
        return <>
            <option value="number">number</option>
            <option value="hall">hall</option>
            <option value="room">room</option>
            <option value="wing">wing</option>
            <option value="role">role</option>
            <option value="first name">first name</option>
            <option value="last name">last name</option>
            <option value="pronouns">pronouns</option>
        </>
    }
    return <>
        <table className='context_menu'>
        <tbody>
            <tr><td>uuid  </td><td><input  value={uuid} onChange={(e) => setUuid(e.target.value)}/> </td></tr>
            <tr><td>field </td><td><select onChange={(e) => setField(e.target.value)}>{Options()}</select></td></tr>
            <tr><td>value </td><td><input  onChange={(e) => setValue(e.target.value)}/>  </td></tr>
            <tr><td></td><td><button onClick={() => {HttpEditStudent(editData)}}>Submit Edit</button></td></tr>
        </tbody>
        </table>
    </>
}

function CreateStudent(): JSX.Element {
    const [state, setState] = useState<FullStudentInfo>({room: 0} as FullStudentInfo);

    return <>
        <table className='context_menu'>
        <tbody>
            <tr><td>first name </td><td><input onChange={(e) => setState({...state, fname:  e.target.value})} /> </td></tr>
            <tr><td>last name </td><td><input onChange={(e) => setState({...state, lname:  e.target.value})} /> </td></tr>
            <tr><td>pronouns </td><td><input onChange={(e) => setState({...state, pronouns:  e.target.value})} /> </td></tr>
            <tr><td>number</td><td><input onChange={(e) => setState({...state, number: parseInt(e.target.value)})} type="number" />  </td></tr>
            <tr><td>hall  </td><td><input onChange={(e) => setState({...state, hall:   e.target.value})} /> </td></tr>
            <tr><td>room  </td><td><input onChange={(e) => setState({...state, room:   parseInt(e.target.value)})} type="number" /> </td></tr>
            <tr><td>wing  </td><td><input onChange={(e) => setState({...state, wing:   e.target.value})} /> </td></tr>
            <tr><td></td><td><button onClick={() => HttpCreateStudent(state)}>Create Student</button></td></tr>
        </tbody>
        </table>
    </>
}

function GetStudent({init_uuid}: {init_uuid: string}): JSX.Element {
    const [uuid, setUuid] = useState(init_uuid)
    const [info, setInfo] = useState(DefaultAllStudentInfo())

    useEffect(() => {
        async function init() {
            if(uuid == "") {
                return
            }
            setInfo(await HttpGetStudent(uuid, true))
        }
        init()
    }, [])

    return <>
        <table className='context_menu'>
        <tbody>
            <tr><td>UUID       </td><td><input value={uuid} onChange={(e) => setUuid(e.target.value)}/></td></tr>
            <tr><td>first name </td><td>{info.fname}</td></tr>
            <tr><td>last name  </td><td>{info.lname}</td></tr>
            <tr><td>pronouns   </td><td>{info.pronouns}</td></tr>
            <tr><td>number     </td><td>{info.number}</td></tr>
            <tr><td>hall       </td><td>{info.hall}</td></tr>
            <tr><td>room       </td><td>{info.room}</td></tr>
            <tr><td>wing       </td><td>{info.wing}</td></tr>
            <tr><td></td><td><button onClick={async () => {setInfo(await HttpGetStudent(uuid, true))}}>Get</button></td></tr>
        </tbody>
        </table>
    </>
}

function DeleteStudent({init_uuid}: {init_uuid: string}): JSX.Element {
    const [uuid, setUuid] = useState(init_uuid)
    const [checked, setChecked] = useState(false)

    return <>
        <table className='context_menu'>
        <tbody>
            <tr><td>UUID         </td><td><input value={uuid} onChange={(e) => setUuid(e.target.value)}/></td></tr>
            <tr><td>Are you sure?</td><td><input type="checkbox" onChange={(e) => setChecked(e.target.checked)} /></td></tr>
            {!checked ? <></> : 
            <tr><td></td><td><button onClick={() => HttpDeleteStudent(uuid)}>Confirm</button></td></tr>
            }
        </tbody>
        </table>
    </>
}

function GrantAccess(): JSX.Element {
    const [state, setState] = useState<TableUsers>({} as TableUsers);

    return <>
        <table className='context_menu'>
        <tbody>
            <tr><td>first name </td><td><input onChange={(e) => setState({...state, fname:  e.target.value})} /> </td></tr>
            <tr><td>last name </td><td><input onChange={(e) => setState({...state, lname:  e.target.value})} /> </td></tr>
            <tr><td>role  </td><td><input onChange={(e) => setState({...state, role:   e.target.value})} /> </td></tr>
            <tr><td></td><td><button onClick={async () => alert(`${window.origin}/signup?token=${(await HttpCreateUser(state)).token}`)}>Create User</button></td></tr>
        </tbody>
        </table>
    </>
}

function EditUser({init_uuid}: {init_uuid: string}): JSX.Element {
    const [editData, setEditData] = useState({} as UpdateUser)
    const [uuid, setUuid] = useState(init_uuid)
    const [field, setField] = useState("first name")
    const [value, setValue] = useState("")

    useEffect(() => {
        setEditData({
            uuid: uuid,
            field: field,
            str_field: value,
        } as UpdateUser);

    }, [uuid, field, value])

    function Options(): JSX.Element {
        return <>
            <option value="first name">first name</option>
            <option value="last name">last name</option>
            <option value="role">role</option>
        </>
    }
    return <>
        <table className='context_menu'>
        <tbody>
            <tr><td>uuid  </td><td><input  value={uuid} onChange={(e) => setUuid(e.target.value)}/> </td></tr>
            <tr><td>field </td><td><select onChange={(e) => setField(e.target.value)}>{Options()}</select></td></tr>
            <tr><td>value </td><td><input  onChange={(e) => setValue(e.target.value)}/>  </td></tr>
            <tr><td></td><td><button onClick={() => {HttpUpdateUser(editData)}}>Submit Edit</button></td></tr>
        </tbody>
        </table>
    </>
}

