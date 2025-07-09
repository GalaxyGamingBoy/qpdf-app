import { RouteRecordRaw } from "vue-router"
import HomeView from "./pages/HomeView.vue"
import PDFMergeView from "./pages/pdf/PDFMergeView.vue";

const routes: RouteRecordRaw[] = [
  {
    path: "/",
    component: HomeView
  },
  {
    path: "/pdf/merge",
    component: PDFMergeView
  }
]

export default routes;