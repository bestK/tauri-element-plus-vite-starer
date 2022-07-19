<template>
    <Codemirror v-model:value="context" :options="cmOptions" border placeholder="" height="100%" width="100%"
        @change="change" />
</template>

<script>
import Codemirror from "codemirror-editor-vue3";

// language
import "codemirror/mode/javascript/javascript.js";

// theme
import "codemirror/theme/dracula.css";

export default {
    props: {
        context: String
    },
    components: { Codemirror },
    setup(props) {
        const context = props.context;
        return {
            context,
            cmOptions: {
                mode: "text/javascript", // Language mode
                theme: "dracula", // Theme
                lineNumbers: true, // Show line number
                smartIndent: true, // Smart indent
                indentUnit: 2, // The smart indent unit is 2 spaces in length
                foldGutter: true, // Code folding
                styleActiveLine: true, // Display the style of the selected row
                fullScreen: true
            },
            change
        };

        function change(e) {
            console.log('change', e)
        }
    },

}

</script>
<style scoped>
.vue-codemirror {
    height: 100%;
}

.CodeMirror {
    height: 100%;
    height: 100%;
}
</style>