import { ryza3 } from "data";

export default function ItemList() {
  return (
    <>
      <h1>Ryza 3 item list</h1>
      <pre>{JSON.stringify(ryza3.item_data, null, 4)}</pre>
    </>
  );
}
