import React, { useEffect } from "react";
import Searchbar from "./components/searchbar";
import MoreSettings from "./components/moresettings";
import Add from "./components/add";
import Save from "./components/save";
import { Modal } from "./components/modal";
import { Row, Rows, AddRow } from "./components/rows";
import { Account, add_account, append_account, get_accounts, save, mexport } from "../utils/invoker";
import { Loading } from "./components/loading";
import { mopen } from "../utils/dialog";

export default function App() {
    const [searchterm, setSearchTerm] = React.useState("");
    const [showModal, setShowModal] = React.useState(false);
    const [accounts,setAccounts] = React.useState<Account[]>([]);
    const [showAddRow, setShowAddRow] = React.useState(false);
    const [showLoading,setShowLoading] = React.useState(false);
    const [reload, setReload] = React.useState(true);
    const options = [
        {
            name: "Append",
            cb: async () => {
                setShowLoading(true);
                setShowModal(true);
                try {
                    let path = await mopen();
                    append_account(path);
                }
                catch (e) {
                    // TODO
                }
                setReload(true);
                setTimeout(() => {
                    setShowLoading(false)
                    setShowModal(false);
                }, 500);
            }
        },
        {
            name: "Export",
            cb: () => {
                mexport();
            }
        },
    ];

    useEffect(() => {
        if (reload) {
            let query = undefined;
            if (searchterm.trim().length != 0) {
                const st = searchterm.trim()
                query = {
                    username: st,
                    link:     st
                };
            }
            // console.log(query);
            get_accounts(query)
                .then((v) => {
                    setAccounts(v);
                    // console.log(accounts);
                });
            setReload(false);
        }
    }, [reload, searchterm]);

    return (
        <div className="w-[80%] m-auto my-6">
            <div className="w-full m-auto my-2 flex flex-row items-center gap-2">
                <MoreSettings
                    options={options}
                    onClick={(_) => {}}
                />
                <Searchbar
                    value={searchterm}
                    classContainer='grow'
                    className='w-full'
                    placeholder='Search'
                    onChange={async (e) => {
                        const v = e.target.value;
                        setSearchTerm(v);
                        setReload(true);
                    }}
                    shortcut="/"
                />
                <Add onClick={(_) => {
                    setShowAddRow(true);
                }} />
                <Save onClick={(_) => {
                    setShowLoading(true);
                    setShowModal(true);
                    try {
                        save();
                    }
                    catch (e) {
                        // TODO
                    }
                    setTimeout(() => {
                        setShowLoading(false)
                        setShowModal(false);
                    }, 500);
                }} />
            </div>
            <Rows>
                <>
                   {
                        (showAddRow)?
                            <AddRow
                                onOk={(v) => {
                                    add_account(v);
                                    setShowAddRow(false);
                                    setReload(true);
                                }}
                                onCancel={() => {
                                    setShowAddRow(false);
                                }}
                            />:<></>
                    }
                    {
                        accounts.map((v, i) => {
                            return (
                                <Row
                                    key={`${v.username}-${i}`}
                                    username={v.username!}
                                    link={v.link!}
                                    password={v.password!}
                                    onDelete={() => {
                                        setReload(true);
                                    }}
                                />
                            )
                        })
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
