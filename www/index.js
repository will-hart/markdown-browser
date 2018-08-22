var receiveFiles = function(files) {
    ractive.set('files', files);
}

var setPreview = function(preview) {
    ractive.set('preview', preview);
}

var rpc = {
    invoke: function(arg) { window.external.invoke(JSON.stringify(arg)); },
    render: receiveFiles,
    renderPreview: setPreview,
    init: function() { rpc.invoke({ cmd: 'init' }); }
}

var ractive = new Ractive({
    target: '#main',
    template: '#main-template',
    data: {
        files: [],
        preview: null,
        previewContent: function(content) {
            if (content.length < 10) return content
            return content.substring(0, 10) + '...'
        }
    },
    on: {
        filter: function(ctx) {
            console.log('Applying filter: ' + ctx.node.value)
        },
        preview: function(ctx, item) {
            console.log('previewing' + item);
            external.invoke('preview', item)
        }
    }
})

window.onload= function() { rpc.init(); };