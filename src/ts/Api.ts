import { invoke } from "@tauri-apps/api/tauri";

export async function getAllWakatimeData() {
  const res = await invoke("get_all_wakatime_data");
  console.table(res);
  return res;
}

export default {};
