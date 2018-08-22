var receiveFiles = function(files) {
    ractive.set('files', files);
}

var renderPreview = function(preview) {
    // alert(preview.contents);
    ractive.set('preview', preview.contents);
    renderMathInElement(document.getElementById("preview"), {
        delimiters: [
            {left: "$$", right: "$$", display: true},
            {left: "$", right: "$", display: false}
        ]
    });
    mark.mark(ractive.get('filter'))
}

var rpc = {
    invoke: function(arg) {
        var toSend = JSON.stringify(arg);
        // alert(toSend);
        window.external.invoke(toSend);
    },
    render: receiveFiles,
    renderPreview: renderPreview,
    requestPreview: function(item) {
        const arg = { cmd: 'preview', contents: item };
        // alert(arg.contents)
        rpc.invoke(arg)
    },
    init: function() { rpc.invoke({ cmd: 'init' }); }
}

var ractive = new Ractive({
    target: '#main',
    template: '#main-template',
    data: {
        files: [],
        preview: null,
        filter: '',
        itemCount: 0
    },
    computed: {
        filtered: function() {
            var filter = this.get('filter')
            if (filter === 'undefined' || filter === null || filter.length === 0) return this.get('files')
            return this.get('files').filter(function(item) {
                if (item.path.indexOf(filter) !== -1) return true
                return item.contents.indexOf(filter) >= 0
            })
        }
    }
})

ractive.on('preview', function(ctx, item) {
    // alert('previewing: ' + item.contents);
    rpc.requestPreview(item.contents)
})

// set up an observer for the file count
ractive.observe('filtered', function(newVal) {
    ractive.set('itemCount', newVal.length)
})

// set up marking the filter text
var mark = new Mark(document.getElementById("preview"))
ractive.observe('filter', function(newVal) {
    mark.unmark();
    mark.mark(newVal);
})

window.onload = function() { rpc.init(); };