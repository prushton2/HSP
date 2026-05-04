import './Admin.css'
import { useEffect, useState, type JSX } from 'react'
import { Http } from '../../axios/axios'
import { type ApiRequestObjects, type ApiResponseObjects, type Tables, DefaultAllStudentInfo } from '../../axios/structs';
import { prompt } from '../../components/Modal';
import HoverDropdown from '../../components/HoverDropdown';
import RenderTable from './RenderTable';
import { Toast } from '../../components/toast';
import { RoleGE } from '../../components/Role';
import DatePicker from 'react-datepicker';
import { formatProperly } from '../../components/Format';

function Admin({user, ribbon}: {user: Tables.Users, ribbon: (e: JSX.Element) => void}) {
    const [studentInfo, setStudentInfo] = useState<ApiResponseObjects.AllTables>({} as ApiResponseObjects.AllTables);
    const [selectedUUID, setSelectedUUID] = useState<string>("");
    
    
    useEffect(() => {
        async function init() {
            let info = await Http.Admin.GetAllTables()
            setStudentInfo(info);
        }
        init();
    }, [])

    useEffect(() => {
        ribbon(Ribbon(user, selectedUUID))
    }, [selectedUUID, user])

    return (
    <>
        <div className="tables">
            <RenderTable select={(u) => {setSelectedUUID(u)}} selected={selectedUUID} info={studentInfo.studentinfo} tag="student_info" />
            <RenderTable select={(u) => {setSelectedUUID(u)}} selected={selectedUUID} info={studentInfo.residence} tag="residencies" />
            <RenderTable select={(u) => {setSelectedUUID(u)}} selected={selectedUUID} info={studentInfo.activities} tag="activities" />
            {
                RoleGE(user.role, "Owner") ? <>
                <RenderTable select={(u) => {setSelectedUUID(u)}} selected={selectedUUID} info={studentInfo.users} tag="users" />
                <RenderTable select={(u) => {setSelectedUUID(u)}} selected={selectedUUID} info={studentInfo.tokens} tag="tokens" />
                </>: <></>
            }
            
        </div>
    </>
    )
}

export default Admin

function Ribbon(user: Tables.Users, selectedUUID: string): JSX.Element {
    return <>
        <HoverDropdown title="Student" buttons={[
            ["Create",  async () => {await prompt.show("Create Student", <CreateStudent />)}],
            ["Edit",    async () => {await prompt.show("Edit Student",   <EditStudent   init_uuid={selectedUUID == "0" ? "" : selectedUUID}/>)}],
            ["Get",     async () => {await prompt.show("Get Student",    <GetStudent    init_uuid={selectedUUID == "0" ? "" : selectedUUID}/>)}],
            ["Delete",  async () => {await prompt.show("Delete Student", <DeleteStudent init_uuid={selectedUUID == "0" ? "" : selectedUUID}/>)}],
        ]}/>

        <HoverDropdown title="Activities" buttons={[
            ["Create", async () => {await prompt.show("Create Activity", <CreateActivity />)}],
            ["Edit",   async () => {await prompt.show("Edit Activity",   <EditActivity init_uuid={selectedUUID == "0" ? "" : selectedUUID}/>)}],
            ["Delete", async () => {await prompt.show("Delete Activity", <DeleteActivity init_uuid={selectedUUID == "0" ? "" : selectedUUID}/>)}],
            ["Assign", async () => {await prompt.show("Assign Students", <BindActivity init_uuid={selectedUUID == "0" ? "" : selectedUUID}/>)}],
            ["View",   async () => {await prompt.show("View Activity",   <GetActivity init_uuid={selectedUUID == "0" ? "" : selectedUUID}/>)}],
        ]}/>
        {
        RoleGE(user.role, "Owner") ? <>
            <HoverDropdown title="Users" buttons={[
                ["Create",   async () => {await prompt.show("Create User", <GrantAccess />)}],
                ["Update",   async () => {await prompt.show("Update User", <EditUser    init_uuid={selectedUUID == "0" ? "" : selectedUUID}/>)}],
                ["Delete",   async () => {await prompt.show("Delete User", <DeleteUser  init_uuid={selectedUUID == "0" ? "" : selectedUUID}/>)}],
            ]}/>

            <HoverDropdown title="Tokens" buttons={[
                ["Grant",  async () => {await prompt.show("Grant Token",   <GrantToken   init_uuid={selectedUUID == "0" ? "" : selectedUUID}/>)}],
                ["Revoke", async () => {await prompt.show("Revoke Tokens", <RevokeTokens init_uuid={selectedUUID == "0" ? "" : selectedUUID}/>)}],
            ]}/>
        
            </>: <></>
        }
    </>
}

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
            <option value="number">Number</option>
            <option value="hall">Hall</option>
            <option value="room">Room</option>
            <option value="wing">Wing</option>
            <option value="role">Role</option>
            <option value="first name">First Name</option>
            <option value="last name">Last Name</option>
            <option value="pronouns">Pronouns</option>
        </>
    }
    return <>
        <table className='context_menu'>
        <tbody>
            <tr><td>UUID  </td><td><input  value={uuid} onChange={(e) => setUuid(e.target.value)}/> </td></tr>
            <tr><td>Field </td><td><select className="select" onChange={(e) => setField(e.target.value)}>{Options()}</select></td></tr>
            <tr><td>Value </td><td><input  onChange={(e) => setValue(e.target.value)}/>  </td></tr>
            <tr><td></td><td><button className="highlight-button" onClick={async () => {
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
            <tr><td>First Name </td><td><input onChange={(e) => setState({...state, fname:  e.target.value})} /> </td></tr>
            <tr><td>Last Name  </td><td><input onChange={(e) => setState({...state, lname:  e.target.value})} /> </td></tr>
            <tr><td>Pronouns   </td><td><input onChange={(e) => setState({...state, pronouns:  e.target.value})} /> </td></tr>
            <tr><td>Number     </td><td><input onChange={(e) => setState({...state, number: parseInt(e.target.value)})} type="number" />  </td></tr>
            <tr><td>Hall       </td><td><input onChange={(e) => setState({...state, hall:   e.target.value})} /> </td></tr>
            <tr><td>Room       </td><td><input onChange={(e) => setState({...state, room:   parseInt(e.target.value)})} type="number" /> </td></tr>
            <tr><td>Wing       </td><td><input onChange={(e) => setState({...state, wing:   e.target.value})} /> </td></tr>
            <tr><td></td><td><button className="highlight-button" onClick={async () => 
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
            <tr><td>First Name </td><td>{info.fname}</td></tr>
            <tr><td>Last Name  </td><td>{info.lname}</td></tr>
            <tr><td>Pronouns   </td><td>{info.pronouns}</td></tr>
            <tr><td>Number     </td><td>{info.number}</td></tr>
            <tr><td>Hall       </td><td>{info.hall}</td></tr>
            <tr><td>Room       </td><td>{info.room}</td></tr>
            <tr><td>Wing       </td><td>{info.wing}</td></tr>
            <tr><td></td><td><button className="highlight-button" onClick={async () => {
                    let result = await Toast.WrapErr(() => Http.Student.Get(uuid, true))
                    if(result.is_ok()) {setInfo(result.into_ok())}
                }}>Get Student</button></td></tr>
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
            <tr><td></td><td><button className={checked ? "highlight-button" : "disabled-button"} onClick={async () => {
                if(!checked) {return}
                await Toast.WrapFunction(() => Http.Student.Delete(uuid))
            }}>Confirm</button></td></tr>
        </tbody>
        </table>
    </>
}

function UrlPopup({url}: {url: string}): JSX.Element {
    const [copied, setCopied] = useState(false);

    useEffect(() => {
        if(copied) {
            setTimeout(() => setCopied(false), 2500)
        }
    }, [copied])

    return <>
        <h4 style={{margin: "0px"}}>Send this URL to the user</h4>
        <label style={{marginBottom: "20px", marginTop: "20px"}}>{url.substring(0, 32)}...</label>
        <button className='highlight-button' onClick={() => {
            navigator.clipboard.writeText(url);
            setCopied(true);
        }}>{copied ? "Copied!" : "Copy"}</button>
    </>
    
}

function GrantAccess(): JSX.Element {
    const [state, setState] = useState<Tables.Users>({} as Tables.Users);

    return <>
        <table className='context_menu'>
        <tbody>
            <tr><td>First Name </td><td><input onChange={(e) => setState({...state, fname:  e.target.value})} /> </td></tr>
            <tr><td>Last Name </td><td><input onChange={(e) => setState({...state, lname:  e.target.value})} /> </td></tr>
            <tr><td>Role </td><td><input onChange={(e) => setState({...state, role:   e.target.value})} /> </td></tr>
            <tr><td></td><td><button className="highlight-button" onClick={async () => {
                    let result = await Toast.WrapErr(() => Http.User.Create(state))
                    if(result.is_ok()) {
                        prompt.show("Login URL", <UrlPopup url={`${window.origin}/signup?token=${result.into_ok().token}`} />)
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
            <tr><td>UUID  </td><td><input  value={uuid} onChange={(e) => setUuid(e.target.value)}/> </td></tr>
            <tr><td>Field </td><td><select onChange={(e) => setField(e.target.value)}>{Options()}</select></td></tr>
            <tr><td>Value </td><td><input  onChange={(e) => setValue(e.target.value)}/>  </td></tr>
            <tr><td></td><td><button className="highlight-button" onClick={async () => {
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
            <tr><td></td><td><button className={checked ? "highlight-button" : "disabled-button"} onClick={async () => {
                if(!checked) {return}
                await Toast.WrapFunction(() => Http.User.Delete(uuid))
            }}>Confirm</button></td></tr>
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
            <tr><td></td><td><button className={checked ? "highlight-button" : "disabled-button"} onClick={async () => {
                if(!checked) {return}
                let result = await Toast.WrapErr(() => Http.User.Token.Grant(uuid));
                if(result.is_ok()) {
                    prompt.show("Login URL", <UrlPopup url={`${window.origin}/signup?token=${result.into_ok().token}`} />)
                }
            }}>Confirm</button></td></tr>
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
            <tr><td></td><td><button className={checked ? "highlight-button" : "disabled-button"} onClick={async () => {
                if(!checked) {return}
                await Toast.WrapFunction(() => Http.User.Token.Revoke(uuid), "All Tokens Revoked")
            }}>Confirm</button></td></tr>
        </tbody>
        </table>
    </>
}

function CreateActivity(): JSX.Element {
    const [state, setState] = useState<ApiRequestObjects.CreateActivity>({} as ApiRequestObjects.CreateActivity);
    const [dates, setDates] = useState<Date[]>([])

    useEffect(() => {
        console.log(dates.map((e) => {return e.getTime()}).join("\n"))
    }, [dates])

    return <>
        <table className='context_menu'>
        <tbody>
            <tr><td>Name  </td><td><input onChange={(e) => setState({...state, name:   e.target.value})} /> </td></tr>
            <tr><td>Staff </td><td><input placeholder="Separate Staff With ;" onChange={(e) => setState({...state, staff:   e.target.value.split(";")})} /> </td></tr>
            <tr><td>Dates </td><td><DatePicker selectsMultiple shouldCloseOnSelect={false} selectedDates={dates} onChange={(dates: any) => setDates(dates as Date[])} /></td></tr>

            <tr><td></td><td><button className="highlight-button" onClick={async () => {
                let object = state;
                object.dates = dates.map<number>((e) => e.getTime())
                await Toast.WrapFunction(() => Http.Activity.Create(object), "Activity Created")
            }}>Create Activity</button></td></tr>
        </tbody>
        </table>
    </>
}

function EditActivity({init_uuid}: {init_uuid: string}): JSX.Element {
    const [uuid, setUuid] = useState(init_uuid)
    const [state, setState] = useState<ApiRequestObjects.EditActivity>({uuid: "", name: null,} as ApiRequestObjects.EditActivity);
    const [dates, setDates] = useState<Date[]>([])

    return <>
        <h5>Fields left blank will not be changed</h5>
        <table className='context_menu'>
        <tbody>
            <tr><td>UUID  </td><td><input value={uuid} onChange={(e) => setUuid(e.target.value)} /> </td></tr>
            <tr><td>Name  </td><td><input onChange={(e) => setState({...state, name:   e.target.value == "" ? null : e.target.value})} /> </td></tr>
            <tr><td>Staff </td><td><input placeholder="Separate Staff With ;" onChange={(e) => setState({...state, staff:   e.target.value == "" ? null : e.target.value.split(";")})} /> </td></tr>
            <tr><td>Dates </td><td><DatePicker selectsMultiple shouldCloseOnSelect={false} selectedDates={dates} onChange={(dates: any) => setDates(dates as Date[])} /></td></tr>

            <tr><td></td><td><button className="highlight-button" onClick={async () => {
                let object = state;
                object.uuid = uuid
                object.dates = dates.length == 0 ? null : dates.map<number>((e) => e.getTime())

                await Toast.WrapFunction(() => Http.Activity.Edit(object), "Activity Updated")
            }}>Edit Activity</button></td></tr>
        </tbody>
        </table>
    </>
}

function BindActivity({init_uuid}: {init_uuid: string}): JSX.Element {
    const [uuid, setUuid] = useState(init_uuid)
    const [students, setStudents] = useState<number[]>([])

    useEffect(() => {
        console.log(students)
    }, [students])

    return <>
        <h5>Enter student numbers <br /> (will overwrite existing assignments)</h5>
        <table className='context_menu'>
        <tbody>
            <tr><td>UUID  </td><td><input value={uuid} onChange={(e) => setUuid(e.target.value)} /> </td></tr>
            <tr><td>Student Numbers </td><td><input onChange={(e) => setStudents(e.target.value.split(/[^0-9]/).filter((v) => v != "").map((v) => parseInt(v)))} /> </td></tr>
            <tr><td /><td><button className='highlight-button' onClick={async () => {
                await Toast.WrapFunction(() => Http.Activity.Assign(uuid, students), "Assigned Students")
            }}>Assign Students</button></td></tr>
        </tbody>
        </table>
    </>
}

function GetActivity({init_uuid}: {init_uuid: string}): JSX.Element {
    const [uuid, setUuid] = useState(init_uuid)
    const [info, setInfo] = useState<Tables.Activity>({uuid: "", name: "", staff: [], dates: []} as Tables.Activity)
    const [students, setStudents] = useState<ApiResponseObjects.FullStudent[]>([])

    useEffect(() => {
        async function init() {
            if(uuid == "") {
                return
            }
            let result = await Toast.WrapErr<[Tables.Activity, ApiResponseObjects.FullStudent[]], string>(() => Http.Activity.Get(uuid))
            if(result.is_ok()) {
                setInfo(result.into_ok()[0])
                setStudents(result.into_ok()[1])
            }
        }
        init()
    }, [])

    return <>
        <table className='context_menu'>
        <tbody>
            <tr>
                <td>UUID</td>
                <td><input value={uuid} onChange={(e) => setUuid(e.target.value)}/></td>
                <td /><td><button onClick={async() => {
                    let result = await Toast.WrapErr<[Tables.Activity, ApiResponseObjects.FullStudent[]], string>(() => Http.Activity.Get(uuid))
                    if(result.is_ok()) {
                        setInfo(result.into_ok()[0])
                        setStudents(result.into_ok()[1])
                    }
                }}>Get Activity</button></td>
            </tr>
        </tbody>
        </table>
        <h2>Activity</h2>
        <table className='context_menu'>
        <thead>
            <tr>
                <td>Activity</td><td>Staff</td><td>Dates</td>
            </tr>
        </thead>
        <tbody>
            <tr>
                <td>{info.name}</td>
                <td>{info.staff.filter((a) => a != "").join(", ")}</td>
                <td><DatePicker selectsMultiple shouldCloseOnSelect={false} selectedDates={info.dates.filter((e) => e != 0).map((e) => new Date(e))} /></td>
            </tr>
        </tbody>
        </table>
        
        <h2>Attendees</h2>
        <table className='context_menu'>
        <thead>
            <tr>
                <td>Name</td>
                <td>Pronouns</td>
                <td>Number</td>
                <td>Residence</td>
            </tr>
        </thead>
        <tbody>
            {
                students.map((e) =>
                <tr>
                    <td>{e.fname} {e.lname}</td>
                    <td>{e.pronouns}</td>
                    <td>{e.number}</td>
                    <td>{formatProperly(e.hall)} {e.room} ({e.wing})</td>
                </tr>
                )
            }
        </tbody>
        </table>
    </>
}

function DeleteActivity({init_uuid}: {init_uuid: string}): JSX.Element {
    const [uuid, setUuid] = useState(init_uuid)
    const [checked, setChecked] = useState(false)

    return <>
        <table className='context_menu'>
        <tbody>
            <tr><td>UUID         </td><td><input value={uuid} onChange={(e) => setUuid(e.target.value)}/></td></tr>
            <tr><td>Are you sure?</td><td><input type="checkbox" onChange={(e) => setChecked(e.target.checked)} /></td></tr>
            <tr><td></td><td><button className={checked ? "highlight-button" : "disabled-button"} onClick={async () => {
                if(!checked) {return}
                await Toast.WrapFunction(() => Http.Activity.Delete(uuid), "User Deleted")
            }}>Confirm</button></td></tr>
        </tbody>
        </table>
    </>
}