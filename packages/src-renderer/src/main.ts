import './style.css'
import logo from './assets/logo.png'
import { listen } from '@tauri-apps/api/event'

const app = document.querySelector<HTMLDivElement>('#app')!

app.innerHTML = `
  <img src="${logo}" />
  <h1>Tauri App Template!</h1>
  <div id="directory-info"></div>
`

interface DirectoryInfo {
  path: string;
  size: number;
  file_count: number;
}

listen<DirectoryInfo>('directory_info', (event: { payload: DirectoryInfo }) => {
  const directoryInfo = event.payload
  const directoryInfoDiv = document.getElementById('directory-info')!
  directoryInfoDiv.innerHTML += `
    <p>${directoryInfo.path} - ${directoryInfo.file_count} files - ${formatSize(directoryInfo.size)}</p>
  `
})

function formatSize (size: number) {
  const KB = 1024
  const MB = KB * 1024
  const GB = MB * 1024

  if (size >= GB) {
    return `${(size / GB).toFixed(2)} GB`
  } else if (size >= MB) {
    return `${(size / MB).toFixed(2)} MB`
  } else if (size >= KB) {
    return `${(size / KB).toFixed(2)} KB`
  } else {
    return `${size.toFixed(2)} bytes`
  }
}
