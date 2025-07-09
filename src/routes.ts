import { RouteRecordRaw } from "vue-router"
import HomeView from "./pages/HomeView.vue"

const routes: RouteRecordRaw[] = [
  {
    path: "/",
    component: HomeView
  }
]

export default routes;