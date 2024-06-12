import React from "react";
import Searchbar from "./components/searchbar";
import MoreSettings from "./components/moresettings";
import Add from "./components/add";
import Save from "./components/save";
import { Modal } from "./components/modal";
import { Row, Rows, AddRow } from "./components/rows";
import { Account, add_account, get_accounts, save } from "../utils/invoker";
import { Loading } from "./components/loading";

function searchV(v: string) {
    console.log(v);
}

export default function App() {
    const [searchbar, setSearchbar] = React.useState("");
    const [showModal, setShowModal] = React.useState(false);
    const [accounts,setAccounts] = React.useState<Account[]>([]);
    const [showAddRow, setShowAddRow] = React.useState(false);
    const [showLoading,setShowLoading] = React.useState(false);
    const options = [
        {
            name: "Append",
            cb: () => {
                console.log("append");
            }
        },
        {
            name: "Export",
            cb: () => {

            }
        },
    ];

    React.useEffect(() => {
        get_accounts()
            .then((v) => {
                setAccounts(v);
            });
    });

    return (
        <div className="w-[80%] m-auto my-6">
            <div className="w-full m-auto my-2 flex flex-row items-center gap-2">
                <MoreSettings
                    options={options}
                    onClick={(_) => {}}
                />
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
                <Add onClick={(_) => {
                    setShowAddRow(true);
                }} />
                <Save onClick={(_) => {
                    setShowLoading(true);
                    setShowModal(true);
                    save();
                    setTimeout(() => {
                        setShowLoading(false)
                        setShowModal(false);
                    }, 500);
                }} />
            </div>
            <Rows>
                <>
                {
                    accounts.map((v, i) => {
                        return (
                            <Row
                                key={i}
                                username={v.username!}
                                link={v.link!}
                                password={v.password!}
                            />
                        )
                    })
                }
                {
                    (showAddRow)?
                    <AddRow
                        onOk={(v) => {
                            add_account(v);
                            setShowAddRow(false);
                        }}
                        onCancel={() => {
                            setShowAddRow(false);
                        }}
                    />:<></>
                }
                </>
            </Rows>
            {
                (showLoading && showModal) ?
                <Modal onClick={(_) => {
                    setShowModal(false);
                    setShowLoading(false);
                }} >
                    <Loading />
                </Modal>:<></>
            }
        </div>
    );
}
