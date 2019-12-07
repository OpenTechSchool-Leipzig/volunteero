<template>
  <div class="volunteero">
    <Bot @showModal="showModal = true" />
    <Modal v-show="showModal" @closeModal="showModal = false" />
  </div>
</template>

<script>
import Bot from "@/components/Bot";
import Modal from "@/components/Modal";

export default {
  components: { Bot, Modal },
  data() {
    return {
      showModal: false
    };
  }
};
</script>

<style>
.volunteero * {
  font-family: Arial, Helvetica, sans-serif;
  box-sizing: border-box;
}
</style>
