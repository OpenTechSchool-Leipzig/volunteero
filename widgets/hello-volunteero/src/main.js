import Vue from "vue";
import wrap from "@vue/web-component-wrapper";
import VueWebComponent from "./Main.vue";

const CustomElement = wrap(Vue, VueWebComponent);

window.customElements.define("hello-volunteero", CustomElement);
