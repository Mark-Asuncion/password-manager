import React from "react";
import Searchbar from "./components/searchbar";
import MoreSettings from "./components/moresettings";
import Add from "./components/add";
import Save from "./components/save";
import { Modal } from "./components/modal";
import { Row, Rows, AddRow } from "./components/rows";

function searchV(v: string) {
    console.log(v);
}

export default function App() {
    const [searchbar, setSearchbar] = React.useState("");
    const [showModal, _setShowModal] = React.useState(false);
    return (
        <div className="w-[80%] m-auto my-6">
            <div className="w-full m-auto my-2 flex flex-row items-center gap-2">
                <MoreSettings onClick={(_) => {}} />
                <Searchbar
                    value={searchbar}
                    classContainer='grow'
                    className='w-full'
                    placeholder='Search'
                    onChange={(e) => {
                        const v = e.target.value;
                        setSearchbar(v);
                        searchV(v);
                    }}
                    shortcut="/"
                />
                <Add  onClick={(_) => {}} />
                <Save  onClick={(_) => {}} />
            </div>
            <Rows>
                <Row username="hello" link="link" password="password" />
                <AddRow />
            </Rows>
            {
                (showModal) ?
                <Modal onClick={(_) => {}} >
                    <p>Hello</p>
                </Modal>:<></>
            }
        </div>
    );
}
