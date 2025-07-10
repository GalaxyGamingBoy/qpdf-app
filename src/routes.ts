import { RouteRecordRaw } from "vue-router"
import HomeView from "./pages/HomeView.vue"
import PDFMergeView from "./pages/pdf/PDFMergeView.vue";
import PDFSplitView from "./pages/pdf/PDFSplitView.vue";

const routes: RouteRecordRaw[] = [
  {
    path: "/",
    component: HomeView
  },
  {
    path: "/pdf/merge",
    component: PDFMergeView
  },
  {
    path: "/pdf/split",
    component: PDFSplitView
  }
]

export default routes;