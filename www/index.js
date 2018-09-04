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

//https://stackoverflow.com/questions/3446170/escape-string-for-use-in-javascript-regex
var escapeRegExp = function(str) {
    return str.replace(/[.*+?^${}()|[\]\\]/g, '\\$&');
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

            var re = new RegExp(escapeRegExp(filter), 'mi')
            return this.get('files').filter(function(item) {
                return re.test(item.path) || re.test(item.contents)
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

// Don't follow footnotes and local anchors, only new links
// but make sure new links open in a new window, hopefully this works
var cancelLinksCallback = function(e) {
    var usedE = e || window.e
    var target = usedE.target || usedE.srcElement
    if (!(target instanceof HTMLAnchorElement)) return true;

    var href = target.getAttribute('href');
    if (href.indexOf('#') !== 0) {
        window.open(href, '_blank')
    }

    e.preventDefault();
    return false;
}

window.onload = function() {
    rpc.init();

    if (this.document.addEventListener)
        document.addEventListener('click', cancelLinksCallback, false);
    else
        document.attachListener('onclick', cancelLinksCallback);
};