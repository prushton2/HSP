import { useEffect, useState, type JSX } from 'react'
import './Admin.css'
import { Http } from '../../axios/axios'
import { type ApiRequestObjects, type ApiResponseObjects, type Tables, DefaultAllStudentInfo } from '../../axios/structs';
import { Modal, prompt } from '../../components/Modal';
import HoverDropdown from '../../components/HoverDropdown';
import RenderTable from './RenderTable';
import { Toast } from '../../components/toast';

function Admin() {
    const [studentInfo, setStudentInfo] = useState<ApiResponseObjects.AllTables>({} as ApiResponseObjects.AllTables);
    const [selectedUUID, setSelectedUUID] = useState<string>("");

    useEffect(() => {
        async function init() {
            let info = await Http.Admin.GetAllTables()
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
                ["Create",  async () => {await prompt.show("Create Student", <CreateStudent />)}],
                ["Edit",    async () => {await prompt.show("Edit Student",   <EditStudent   init_uuid={selectedUUID == "0" ? "" : selectedUUID}/>)}],
                ["Get",     async () => {await prompt.show("Get Student",    <GetStudent    init_uuid={selectedUUID == "0" ? "" : selectedUUID}/>)}],
                ["Delete",  async () => {await prompt.show("Delete Student", <DeleteStudent init_uuid={selectedUUID == "0" ? "" : selectedUUID}/>)}],
            ]}/>

            <HoverDropdown title="Activities" buttons={[
                ["Create", async () => {}],
                ["Clone",  async () => {}],
                ["Assign", async () => {}],
                ["Edit",   async () => {}]
            ]}/>

            <HoverDropdown title="Users" buttons={[
                ["Create",   async () => {await prompt.show("Create User", <GrantAccess />)}],
                ["Update",   async () => {await prompt.show("Update User", <EditUser    init_uuid={selectedUUID == "0" ? "" : selectedUUID}/>)}],
                ["Delete",   async () => {await prompt.show("Delete User", <DeleteUser  init_uuid={selectedUUID == "0" ? "" : selectedUUID}/>)}],
            ]}/>

            <HoverDropdown title="Tokens" buttons={[
                ["Grant",  async () => {await prompt.show("Grant Token",   <GrantToken   init_uuid={selectedUUID == "0" ? "" : selectedUUID}/>)}],
                ["Revoke", async () => {await prompt.show("Revoke Tokens", <RevokeTokens init_uuid={selectedUUID == "0" ? "" : selectedUUID}/>)}],
            ]}/>
        </div>
        <div className="tables">
            <RenderTable select={(u) => {setSelectedUUID(u)}} selected={selectedUUID} info={studentInfo.studentinfo} tag="student_info" />
            <RenderTable select={(u) => {setSelectedUUID(u)}} selected={selectedUUID} info={studentInfo.residence} tag="residencies" />
            <RenderTable select={(u) => {setSelectedUUID(u)}} selected={selectedUUID} info={studentInfo.users} tag="users" />
            <RenderTable select={(u) => {setSelectedUUID(u)}} selected={selectedUUID} info={studentInfo.tokens} tag="tokens" />
        </div>
    </>
    )
}

export default Admin

function EditStudent({init_uuid}: {init_uuid: string}): JSX.Element {
    const [editData, setEditData] = useState({} as ApiRequestObjects.EditStudent)
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
        } as ApiRequestObjects.EditStudent);

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
            <tr><td></td><td><button onClick={async () => {
                    await Toast.WrapFunction(() => Http.Student.Edit(editData), "Student Edited")
                }}>Submit Edit</button></td></tr>
        </tbody>
        </table>
    </>
}

function CreateStudent(): JSX.Element {
    const [state, setState] = useState<ApiRequestObjects.CreateStudent>({room: 0} as ApiRequestObjects.CreateStudent);

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
            <tr><td></td><td><button onClick={async () => 
                    await Toast.WrapFunction(() => Http.Student.Create(state), "Student Created")
                }>Create Student</button></td></tr>
        </tbody>
        </table>
    </>
}

function GetStudent({init_uuid}: {init_uuid: string}): JSX.Element {
    const [uuid, setUuid] = useState(init_uuid)
    const [info, setInfo] = useState<ApiResponseObjects.FullStudent>(DefaultAllStudentInfo())

    useEffect(() => {
        async function init() {
            if(uuid == "") {
                return
            }
            let result = await Toast.WrapErr<ApiResponseObjects.FullStudent, string>(() => Http.Student.Get(uuid, true))
            if(result.is_ok()) {setInfo(result.into_ok())}
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
            <tr><td></td><td><button onClick={async () => {
                    let result = await Toast.WrapErr(() => Http.Student.Get(uuid, true))
                    if(result.is_ok()) {setInfo(result.into_ok())}
                }}>Get</button></td></tr>
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
            <tr><td></td><td><button onClick={async () => 
                    await Toast.WrapFunction(() => Http.Student.Delete(uuid))
                }>Confirm</button></td></tr>
            }
        </tbody>
        </table>
    </>
}

function GrantAccess(): JSX.Element {
    const [state, setState] = useState<Tables.Users>({} as Tables.Users);

    return <>
        <table className='context_menu'>
        <tbody>
            <tr><td>first name </td><td><input onChange={(e) => setState({...state, fname:  e.target.value})} /> </td></tr>
            <tr><td>last name </td><td><input onChange={(e) => setState({...state, lname:  e.target.value})} /> </td></tr>
            <tr><td>role  </td><td><input onChange={(e) => setState({...state, role:   e.target.value})} /> </td></tr>
            <tr><td></td><td><button onClick={async () => {
                    let result = await Toast.WrapErr(() => Http.User.Create(state))
                    if(result.is_ok()) {
                        alert(`${window.origin}/signup?token=${result.into_ok().token}`)
                    }
                }}>Create User</button></td></tr>
        </tbody>
        </table>
    </>
}

function EditUser({init_uuid}: {init_uuid: string}): JSX.Element {
    const [editData, setEditData] = useState({} as ApiRequestObjects.EditUser)
    const [uuid, setUuid] = useState(init_uuid)
    const [field, setField] = useState("first name")
    const [value, setValue] = useState("")

    useEffect(() => {
        setEditData({
            uuid: uuid,
            field: field,
            str_field: value,
        } as ApiRequestObjects.EditUser);

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
            <tr><td></td><td><button onClick={async () => {
                    await Toast.WrapFunction(() => Http.User.Update(editData), "User Edited")
                }}>Submit Edit</button></td></tr>
        </tbody>
        </table>
    </>
}

function DeleteUser({init_uuid}: {init_uuid: string}): JSX.Element {
    const [uuid, setUuid] = useState(init_uuid)
    const [checked, setChecked] = useState(false)

    return <>
        <table className='context_menu'>
        <tbody>
            <tr><td>UUID         </td><td><input value={uuid} onChange={(e) => setUuid(e.target.value)}/></td></tr>
            <tr><td>Are you sure?</td><td><input type="checkbox" onChange={(e) => setChecked(e.target.checked)} /></td></tr>
            {!checked ? <></> : 
            <tr><td></td><td><button onClick={async () => 
                    await Toast.WrapFunction(() => Http.User.Delete(uuid), "User Deleted")
                }>Confirm</button></td></tr>
            }
        </tbody>
        </table>
    </>
}

function GrantToken({init_uuid}: {init_uuid: string}): JSX.Element {
    const [uuid, setUuid] = useState(init_uuid)
    const [checked, setChecked] = useState(false)

    return <>
        <table className='context_menu'>
        <tbody>
            <tr><td>UUID         </td><td><input value={uuid} onChange={(e) => setUuid(e.target.value)}/></td></tr>
            <tr><td>Are you sure?</td><td><input type="checkbox" onChange={(e) => setChecked(e.target.checked)} /></td></tr>
            {!checked ? <></> : 
            <tr><td></td><td><button onClick={async () => {
                    let result = await Toast.WrapErr(() => Http.User.Token.Grant(uuid));
                    if(result.is_ok()) {
                        alert(`${window.origin}/signup?token=${result.into_ok().token}`)
                    }
                }}>Confirm</button></td></tr>
            }
        </tbody>
        </table>
    </>
}

function RevokeTokens({init_uuid}: {init_uuid: string}): JSX.Element {
    const [uuid, setUuid] = useState(init_uuid)
    const [checked, setChecked] = useState(false)

    return <>
        <table className='context_menu'>
        <tbody>
            <tr><td>UUID         </td><td><input value={uuid} onChange={(e) => setUuid(e.target.value)}/></td></tr>
            <tr><td>Are you sure?</td><td><input type="checkbox" onChange={(e) => setChecked(e.target.checked)} /></td></tr>
            {!checked ? <></> : 
            <tr><td></td><td><button onClick={async () => {
                    await Toast.WrapFunction(() => Http.User.Token.Revoke(uuid), "All Tokens Revoked")
                }}>Confirm</button></td></tr>
            }
        </tbody>
        </table>
    </>
}