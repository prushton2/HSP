import { useEffect, useState, type JSX } from 'react'
import './Admin.css'
import { GetAllStudentInfo, HttpCreateStudent, HttpEditStudent } from '../../axios/axios'
import { type CreateStudent, type EditStudent, type StudentTablesResponse, type TableActivities, type TableResidencies, type TableStudentActivities, type TableStudentInfo } from '../../axios/structs';
import { Modal, prompt } from '../../components/Modal';

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
                ["Create", async () => {await prompt.show("Create Student", <CreateStudent/>)}],
                ["Edit", async () => {await prompt.show("Edit Student", <EditStudent init_uuid={selectedUUID == "0" ? "" : selectedUUID}/>)}],
                ["Decrypt", () => {}],
                ["Delete", () => {}]
            ]}/>

            <HoverDropdown title="Activities" buttons={[
                ["Create", () => {}],
                ["Clone", () => {}],
                ["Assign", () => {}],
                ["Edit", () => {}]
            ]}/>

            <HoverDropdown title="Access" buttons={[
                ["Grant", () => {}],
                ["Update", () => {}],
                ["Revoke", () => {}],
            ]}/>
        </div>
        <div className="tables">
            <RenderTable select={(u) => {setSelectedUUID(u)}} selected={selectedUUID} info={studentInfo.student_info} tag="student_info" />
            <RenderTable select={(u) => {setSelectedUUID(u)}} selected={selectedUUID} info={studentInfo.residencies} tag="residencies" />
            <RenderTable select={(u) => {setSelectedUUID(u)}} selected={selectedUUID} info={studentInfo.student_activities} tag="student_activities" />
            <RenderTable select={(u) => {setSelectedUUID(u)}} selected={selectedUUID} info={studentInfo.activities} tag="activities" />
        </div>
    </>
    )
}

export default Admin

function HoverDropdown({title, buttons}: {title: string, buttons: [string, () => void][]}) {
  const [isOpen, setIsOpen] = useState(false);

  let button_html: JSX.Element[] = []

  buttons.forEach(([title, fn]) => {
    button_html.push(
        <li><button className='dropdown-element-button' onClick={fn}>{title}</button></li>
    )
  })

  return (
    <div
      style={{ position: "relative", display: "inline-block" }}
      onMouseEnter={() => setIsOpen(true)}
      onMouseLeave={() => setIsOpen(false)}
    >
      {/* Trigger */}
      <button className="dropdown-button">{title}</button>

      {/* Dropdown */}
      {isOpen && (
        <ul className="dropdown-ul">
            {button_html}
        </ul>
      )}
    </div>
  );
}

function RenderTable({info, tag, select, selected}: {select: (uuid: string) => void, selected: string, info: TableResidencies[] | TableStudentActivities[] | TableActivities[] | TableStudentInfo[], tag: string}): JSX.Element {
    const [visible, setVisible] = useState<boolean>(false)
    
    let table_rows: JSX.Element[] = [];
    let head: JSX.Element = <></>;

    if (info == undefined) {
        return <></>
    }

    switch (tag) {
        case "student_info":
            head = <tr>
                <th>UUID</th>
                <th>Number</th>
            </tr>
            break;
        case "residencies":
            head = <tr>
                <th>UUID</th>
                <th>hall</th>
                <th>room</th>
                <th>wing</th>
                <th>role</th>
            </tr>
            break;
        case "student_activities":
            head = <tr>
                <th>UUID</th>
                <th>date</th>
                <th>activity</th>
            </tr>
            break;
        case "activities":
            head = <tr>
                <th>activity</th>
                <th>date</th>
                <th>staff</th>
            </tr>
            break;

    }
    
    info.forEach((row) => {
        let table_row = <></>
        switch (tag) {
            case "student_info":
                table_row = <>
                    <td>{(row as TableStudentInfo).uuid}</td>
                    <td>{(row as TableStudentInfo).number}</td>
                </>
                break;
            case "residencies":
                table_row = <>
                    <td>{(row as TableResidencies).uuid}</td>
                    <td>{(row as TableResidencies).hall}</td>
                    <td>{(row as TableResidencies).room}</td>
                    <td>{(row as TableResidencies).wing}</td>
                    <td>{(row as TableResidencies).role}</td>
                </>
                break;
            case "student_activities":
                table_row = <>
                    <td>{(row as TableStudentActivities).uuid}</td>
                    <td>{(row as TableStudentActivities).date}</td>
                    <td>{(row as TableStudentActivities).activity}</td>
                </>
                break;
            case "activities":
                table_row = <>
                    <td>{(row as TableActivities).activity}</td>
                    <td>{(row as TableActivities).date}</td>
                    <td>{(row as TableActivities).staff}</td>
                </>
                break;
        }

        if(tag == "activities") {
            table_rows.push(
                <tr>
                    {table_row}
                </tr>
            );
        } else {
            table_rows.push(
                <tr 
                    onClick={() => {if (selected == (row as any).uuid) {select("0")} else {select((row as any).uuid)}}}
                    className={selected == (row as any).uuid ? "highlighted_row" : ""}
                >
                    {table_row}
                </tr>
            );
        }
    });

    return <>
        <h2 onClick={() => setVisible(!visible)}>{formatProperly(tag)}</h2>
        <table>
            <thead onClick={() => setVisible(!visible)}>
                {head}
            </thead>
            {visible ? <tbody>
                {table_rows}
            </tbody> : <></>}
        </table>
    </>
}

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
        <table className='edit_student'>
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
    const [state, setState] = useState<CreateStudent>({room: 0} as CreateStudent);

    return <>
        <table className='create_student'>
        <tbody>
            <tr><td>first name </td><td><input onChange={(e) => setState({...state, fname:  e.target.value})} /> </td></tr>
            <tr><td>last name </td><td><input onChange={(e) => setState({...state, lname:  e.target.value})} /> </td></tr>
            <tr><td>number</td><td><input onChange={(e) => setState({...state, number: parseInt(e.target.value)})} type="number" />  </td></tr>
            <tr><td>hall  </td><td><input onChange={(e) => setState({...state, hall:   e.target.value})} /> </td></tr>
            <tr><td>room  </td><td><input onChange={(e) => setState({...state, room:   parseInt(e.target.value)})} type="number" /> </td></tr>
            <tr><td>wing  </td><td><input onChange={(e) => setState({...state, wing:   e.target.value})} /> </td></tr>
            <tr><td>role  </td><td><input onChange={(e) => setState({...state, role:   e.target.value})} /> </td></tr>
            <tr><td></td><td><button onClick={() => HttpCreateStudent(state)}>Create Student</button></td></tr>
        </tbody>
        </table>
    </>
}

function formatProperly(s: string): string {
    s = s.replaceAll("_", " ");
    s = s.charAt(0).toUpperCase() + s.substring(1);

    let news = s

    for(let i = 0; i < s.length-1; i++) {
        if (s[i] == " ") {
            news = s.substring(0, i+1) + s[i+1].toUpperCase() + s.substring(i+2);
        }
    }

    return news;
}