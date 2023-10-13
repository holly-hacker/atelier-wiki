import types from "@/data/types/sophie";
import { Link } from "react-router-dom";
import { itemCategoryDisplayName, itemDisplayName } from "../sophie_data_util";
import { useContext } from "react";
import { SophieContext } from "@/data/sophie_data";

export function ItemLink({
  item,
  children,
}: {
  item: types.Item;
  children?: React.ReactNode;
}) {
  const sophieData = useContext(SophieContext);

  let id;
  if (item.tag) {
    // if the item has a tag, use that
    id = item.tag;

    // strip the 'ITEM_' prefix from the tag, it is present on all items
    id = id.replace(/^ITEM_/, "");
  } else {
    // use the item index
    id = String(sophieData.items.findIndex((v) => v === item));
  }

  return (
    <Link to={`/sophie/items/${id}`}>{children || itemDisplayName(item)}</Link>
  );
}

export function CategoryLink({
  category_tag,
  children,
}: {
  category_tag: string;
  children?: React.ReactNode;
}) {
  const sophieData = useContext(SophieContext);
  const short_category_tag = category_tag.replace(/^ITEM_CATEGORY_/, "");

  return (
    <Link to={`/sophie/item_categories/${short_category_tag}`}>
      {children || itemCategoryDisplayName(sophieData, category_tag)}
    </Link>
  );
}
