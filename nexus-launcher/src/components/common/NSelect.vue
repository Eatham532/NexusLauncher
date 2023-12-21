<!-- Reference https://jsfiddle.net/BB3JK/47/ -->

<script setup lang="ts">
import {computed, Ref, ref} from "vue";

  const props = defineProps({
    options: {
      default: [],
      type: Array as () => any[],
    },
    value: {
      type: String,
      default: "",
    },
    search: {
      type: Boolean,
      default: false,
    },
    filter: {
      type: Boolean,
      default: false,
    },
    blockInvalid: {
      type: Boolean,
      default: false,
    },
    placeholder: {
      type: String,
      default: "",
    },
    noneSearchText: {
      type: String,
      default: "Could not find any options that match your search",
    },
    alwaysShowScrollbar: {
      type: Boolean,
      default: false,
    }
  });

  const emit = defineEmits(['update:value', 'change'])


  const input = ref();
  const textBox = ref();
  const arrow = ref();

  const modified_options: Ref<any[]> = computed(() => {

    if (!props.search || !props.filter) {
      return props.options;
    }

    let new_list: any[] = [];

    props.options.forEach((item) => {
      if (props.value != '') {
        if (item.includes(props.value)) {
          new_list.push(item);
        }
      }
      else {
        new_list.push(item);
      }
    })

    if (props.value != '') {
      return new_list.sort();
    }

    return new_list;
  });

  const createModifiedOptions = () => {
    let new_list: any[] = [];

    props.options.forEach((item) => {
      if (props.value != '') {
        if (item.includes(props.value)) {
          new_list.push(item);
        }
      }
      else {
        new_list.push(item);
      }
    })

    return new_list;
  }

  const active = ref(false);
  const selected = ref();
  const showScrollbar = ref(props.alwaysShowScrollbar);


  const isPosTop = computed(() => {
    return (window.innerHeight / 2) < textBox.value.getBoundingClientRect().top;
  });

  const updateValue = (event: any) => {
    console.log("Updating value");
    emit('update:value', event.target.value);


    if (props.blockInvalid && props.options?.includes(event.target.value)) {
      console.log("Setting selected:" + event.target.value);
      selected.value = event.target.value;
      emit('change');
    }
    else if (!props.blockInvalid) {
      console.log("Setting selected:" + event.target.value);
      selected.value = event.target.value;
      emit('change');
    }
    scrollSelectedIntoView();
  }

  const setValue = (value: any) => {
    console.log("Updating value:"  + value);
    emit('update:value', value);
    emit('change');

    if (props.blockInvalid && props.options?.includes(value)) {
      console.log("Setting selected:" + value);
      selected.value = value;
      emit('change');
    }
    else if (!props.blockInvalid) {
      selected.value = value;
    }
  }

  const textboxBlur = (event: any) => {
    console.log("blur");
    if (event.relatedTarget) {
      if (event.relatedTarget != textBox.value && event.relatedTarget != arrow.value && !event.relatedTarget.classList.contains("dropdown-option")) {
        console.log("None");
        if (props.blockInvalid && props.options?.includes(props.value)) {
          console.log("Setting selected:" + props.value);
          selected.value = props.value;
          setValue(selected.value);
        }

        if (!props.options?.includes(props.value)) {
          setValue(selected.value);
        }
      }
      else {
        console.log("Else");
      }
    }
    else {
      if (props.blockInvalid && props.options?.includes(props.value)) {
        console.log("Setting selected:" + props.value);
        selected.value = props.value;
      }
      active.value = false

      if (!props.options?.includes(props.value)) {
        setValue(selected.value);
      }
    }
  }

  const scrollSelectedIntoView = () => {
    let element = document.getElementsByClassName('option-selected')[0];
    element.scrollIntoView({behavior: "smooth", block: 'nearest', inline: 'start'});
    console.log("Scroll");
  }
</script>

<template>
  <div class="custom-select">
    <div ref="textBox" tabindex="-1" :class="{'text-box':true, 'active': active}" @click.self="active = !active; active ? input.focus() : null;">
      <input ref="input" class="input" type="text" :value="value" @input="updateValue" :placeholder="placeholder" @focus="active = true" :readonly="!search" @blur="textboxBlur" @keydown.enter="active = false; input.blur()" @keydown.tab="active=false">

      <svg @click.prevent="active = !active; active ? input.focus() : null;" ref="arrow" :class="{'arrow':true, 'active': active}" width="0.6em" fill="#000000" version="1.1" id="Capa_1" xmlns="http://www.w3.org/2000/svg" xmlns:xlink="http://www.w3.org/1999/xlink" viewBox="0 0 123.959 123.959" xml:space="preserve">
        <g>
          <path d="M66.18,29.742c-2.301-2.3-6.101-2.3-8.401,0l-56,56c-3.8,3.801-1.1,10.2,4.2,10.2h112c5.3,0,8-6.399,4.2-10.2L66.18,29.742"/>
        </g>
      </svg>
    </div>

    <div ref="dropdown" :class="{'select-dropdown': true, 'top': isPosTop, 'show-scrollbar': showScrollbar}" v-if="active" @mouseover="showScrollbar = true" @mouseleave="!alwaysShowScrollbar ? showScrollbar = false : null">
      <ul>
        <li @click="setValue(option); active = false" tabindex="-1" :class="{'dropdown-option': true, 'option-selected': selected == option}" v-for="option in modified_options">{{ option }}</li>
        <li @click="active = false" tabindex="-1" class='dropdown-option' v-if="!modified_options.length">{{ noneSearchText }}</li>
      </ul>
    </div>
  </div>
</template>

<style lang="stylus" scoped>
.custom-select {
  position: relative;
  max-width: 100%;
  color: #000;
}

.text-box {
  cursor pointer;
  display flex;
  border-width: 0px;
  border-color: #CCCCCC;
  background-color: var(--gray-100);
  color: #000000;
  border-style: solid;
  border-radius: 5px;
  box-shadow: 1px 1px 3px rgba(66,66,66,.75);
  padding 2px;

  &:is(.active) {
    padding 0;
    border 2px black solid;
  }
}

.input {
  background-color transparent;
  width: 100%;
  padding: 0.675em 1em
  cursor: pointer;
  border none;
}

.input:focus {
  outline none;
  selector
}

.selected-value {
  text-align: left;
}

.arrow {
  margin-right 15px;
  margin-left 15px;

  &:is(.active) {
    transform: rotate(180deg);
  }
}



.select-dropdown {
  position: absolute;
  list-style: none;
  width: 100%;
  box-shadow: 0 10px 25px rgba(0, 0, 0, 0.2);
  background-color: #fff;
  border: 1px solid #caced1;
  border-radius: 4px;
  padding: 10px;
  margin-top: 10px;
  margin-bottom: 10px;
  max-height: 200px;
  overflow-y: hidden;
  z-index 99999;
  transition: 0.5s ease;
  transform: scaleY(1);

  &:is(.top) {
    bottom: 100%;
  }

  &:is(.show-scrollbar) {
    overflow-y: auto;
  }
}

.select-dropdown:focus-within {
  box-shadow: 0 10px 25px rgba(94, 108, 233, 0.6);
}

.select-dropdown li {
  position: relative;
  cursor: pointer;
  display: flex;
  align-items: center;
  width: 100%;
  padding: 2px 5px;
  border-radius 4px;
}

.select-dropdown::-webkit-scrollbar {
  width: 7px;
}
.select-dropdown::-webkit-scrollbar-track {
  background: #f1f1f1;
  border-radius: 25px;
}

.select-dropdown::-webkit-scrollbar-thumb {
  background: #ccc;
  border-radius: 25px;
}

.dropdown-option:hover {
  background-color: #f2f2f2;
}

.option-selected {
  background-color var(--gray-300);

}

.select-dropdown input:focus ~ label {
  background-color: #dfdfdf;
}

.select-dropdown input[type="radio"] {
  position: absolute;
  left: 0;
  opacity: 0;
}

* {
  box-sizing: border-box;
}

</style>