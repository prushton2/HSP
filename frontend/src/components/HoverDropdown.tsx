import './HoverDropdown.css'
import { useState, type JSX } from "react";


export default function HoverDropdown({title, buttons}: {title: string, buttons: [string, () => void][]}) {
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
      <button className={`dropdown-button${isOpen ? "-open" : ""}`}>{title}</button>

      {/* Dropdown */}
      {isOpen && (
        <ul className="dropdown-ul">
            {button_html}
        </ul>
      )}
    </div>
  );
}