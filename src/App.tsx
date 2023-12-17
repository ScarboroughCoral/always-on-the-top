import { useEffect, useMemo, useState } from "react";
import reactLogo from "./assets/react.svg";
import { invoke } from "@tauri-apps/api/tauri";
import { Flex, Select, Table, TableColumnsType } from 'antd'
import "./App.css";
type WindowDetail = {
  title: string
  hwnd: number
}
type Response = {
  windows_detail: WindowDetail[]
}
function App() {
  const [windows, setWindows] = useState<WindowDetail[]>([])

  useEffect(() => {
    const fn = (async () => {
      setWindows((await invoke<Response>('get_window_list')).windows_detail)
    })
    setInterval(fn, 3000)
  },[])
  const columns: TableColumnsType<WindowDetail> = useMemo(() => {
    return [
      {
        title: '窗口',
        dataIndex: 'title'
      }
    ]
  }, [])

  return (
    <Table
        rowSelection={{
          type: 'checkbox',
        }}
        columns={columns}
        dataSource={windows}
      />
  );
}

export default App;
