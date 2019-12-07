<template>
  <div>
    <Bot />
    <Modal v-show="showModal" />
  </div>
</template>

<script>
import Bot from "@/components/Bot.vue";
import Modal from "@/components/Modal.vue";

export default {
  components: { Bot, Modal },
  data() {
    return {
      showModal: false
    };
  }
};
</script>
