import { RouteRecordRaw } from "vue-router"
import HomeView from "./pages/HomeView.vue"
import PDFMergeView from "./pages/pdf/PDFMergeView.vue";
import PDFSplitView from "./pages/pdf/PDFSplitView.vue";
import PDFReorderView from "./pages/pdf/PDFReorderView.vue";

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
  },
  {
    path: "/pdf/reorder",
    component: PDFReorderView
  }
]

export default routes;