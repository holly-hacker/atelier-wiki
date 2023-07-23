import { useParams } from "react-router-dom";

export default function ItemDetail() {
  const { id } = useParams();

  let id_num = Number(id);

  if (!id_num) {
    return <>Item not found</>;
  }

  return <>Ryza 3 item detail for item {id_num}</>;
}
