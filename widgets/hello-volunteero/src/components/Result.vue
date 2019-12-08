<template>
  <div class="result">
    <h3>{{ job_description }}</h3>
    <span class="sub">{{ organisation.name }}</span>
    <br />
    <span v-if="disciplines && disciplines.length">Sportart: {{ disciplines }}</span>

    <div class="contact">
      <div v-if="hasContactInfo">
        <h4>Contact</h4>
        {{ contact.name }}
        <div v-if="email && email.length">
          E-Mail:
          {{ email }}
          <span v-if="emailnote && emailnote.length">({{ emailnote }})</span>
        </div>
        <div v-if="phone && phone.length">
          Phone:
          {{ phone }}
          <span v-if="phonenote && phonenote.length">({{ phonenote }})</span>
        </div>
      </div>
      <div v-if="hasLocationsInfo">
        <h4>Ort</h4>
        <span v-for="(location, index) in this.locations" v-bind:key="index">
          {{ location.name }}
          <br />
          {{ location.street }} {{ location.house_number }}
          <br />
          {{ location.postcode }} {{ location.city }}
          <br />
          <br />
        </span>
      </div>
    </div>
  </div>
</template>

<script>
export default {
  props: {
    job_description: String,
    organisation: Object,
    contact: Object,
    labels: Array,
    locations: Array
  },
  data() {
    return {
      disciplineKey: "sport"
    };
  },
  computed: {
    hasContactInfo() {
      return this.contact.options;
    },
    hasLocationsInfo() {
      return this.locations;
    },
    disciplines() {
      const disciplines = this.getLabelValue(this.disciplineKey);

      if (disciplines[0].values) {
        return disciplines[0].values.join(",");
      }

      return "";
    },
    email() {
      if (this.getContactValue("email")) {
        return this.getContactValue("email").value;
      }
      return "";
    },
    phone() {
      if (this.getContactValue("phone")) {
        return this.getContactValue("phone").value;
      }
      return "";
    },
    emailnote() {
      if (this.getContactValue("email")) {
        return this.getContactValue("email").note;
      }
      return "";
    },
    phonenote() {
      if (this.getContactValue("phone")) {
        return this.getContactValue("phone").note;
      }
      return "";
    }
  },
  methods: {
    getLabelValue(key) {
      return this.labels.filter(label => {
        return label.key === key;
      });
    },
    getContactValue(key) {
      if (!this.contact.options) {
        return "";
      }
      const option = this.contact.options.filter(option => {
        if (option[key]) {
          return true;
        }
        return false;
      });

      if (!option[0][key]) {
        return {};
      }
      return option[0][key];
    }
  }
};
</script>

<style lang="scss">
h3,
h4 {
  margin-bottom: 0px;
}
.sub {
  font-size: 1em;
  color: gray;
}
.contact {
  font-size: 0.9em;
}
.result {
  position: absolute;
  top: 0;
  left: 0;
  padding: 0 60px;
  width: 100%;
  transition: all 0.3s;
  &.v-enter {
    transform: translateX(200%);
    opacity: 15%;
  }

  &.v-leave-to {
    transform: translateX(-200%);
    opacity: 15%;
  }
  &.v-enter-to,
  &.v-leave {
    transform: translateX(0);
    opacity: 100%;
  }
  &.v-enter-active,
  &.v-leave-active,
  &.v-move {
    transition: all 0.5s ease-out;
  }
}
</style>
