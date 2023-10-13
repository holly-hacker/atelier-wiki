import { Route } from "react-router-dom";

export default function Routes() {
  return (
    <Route path="sophie">
      <Route index element={<>Index</>} />
    </Route>
  );
}
