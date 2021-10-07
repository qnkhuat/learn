// Compiled by ClojureScript 1.10.879 {:static-fns true, :optimize-constants true, :optimizations :advanced}
goog.provide('joy.music');
goog.require('cljs.core');
goog.require('cljs.core.constants');
/**
 * Return a gain node that goes from silent at time <delay>
 *   up to <volume> in 50 milliseconds, then ramps back down
 *   to silent after <duration>
 */
joy.music.soft_attack = (function joy$music$soft_attack(ctx,p__6025){
var map__6026 = p__6025;
var map__6026__$1 = cljs.core.__destructure_map(map__6026);
var volume = cljs.core.get.cljs$core$IFn$_invoke$arity$2(map__6026__$1,cljs.core.cst$kw$volume);
var delay = cljs.core.get.cljs$core$IFn$_invoke$arity$2(map__6026__$1,cljs.core.cst$kw$delay);
var duration = cljs.core.get.cljs$core$IFn$_invoke$arity$2(map__6026__$1,cljs.core.cst$kw$duration);
var node = ctx.createGainNode();
var G__6027_6028 = node.gain;
G__6027_6028.linearRampToValueAtTime((0),delay);

G__6027_6028.linearRampToValueAtTime(volume,(delay + 0.05));

G__6027_6028.linearRampToValueAtTime((0),(delay + duration));


return node;
});
/**
 * Return an oscillator that plays starting at
 *   <delay> for <duration> seconds
 */
joy.music.sine_tone = (function joy$music$sine_tone(ctx,p__6029){
var map__6030 = p__6029;
var map__6030__$1 = cljs.core.__destructure_map(map__6030);
var cent = cljs.core.get.cljs$core$IFn$_invoke$arity$2(map__6030__$1,cljs.core.cst$kw$cent);
var delay = cljs.core.get.cljs$core$IFn$_invoke$arity$2(map__6030__$1,cljs.core.cst$kw$delay);
var duration = cljs.core.get.cljs$core$IFn$_invoke$arity$2(map__6030__$1,cljs.core.cst$kw$duration);
var node = ctx.createOscillator();
(node.frequency.value = (440));

(node.detune.value = (cent - (900)));

node.noteOn(delay);

node.noteOff((delay + duration));

return node;
});
/**
 * Connect the output of node1 to the input of node2,
 *   returning node2
 */
joy.music.connect_to = (function joy$music$connect_to(node1,node2){
node1.connect(node2);

return node2;
});
/**
 * Play a 'woo' sound; sounds a bit like a glass harp.
 */
joy.music.woo = (function joy$music$woo(ctx,note){
var linger = 1.5;
var note__$1 = cljs.core.update_in.cljs$core$IFn$_invoke$arity$4(note,new cljs.core.PersistentVector(null, 1, 5, cljs.core.PersistentVector.EMPTY_NODE, [cljs.core.cst$kw$duration], null),cljs.core._STAR_,linger);
return joy.music.connect_to(joy.music.sine_tone(ctx,note__$1),joy.music.soft_attack(ctx,note__$1));
});
joy.music.make_once = cljs.core.memoize((function (ctor){
return (new ctor());
}));
/**
 * Kick off playing a sequence of notes. note-fn must take
 *   two arguments, an AudioContext object and a map
 *   representing one note to play. It must return an AudioNode
 *   object that will play that note.
 */
joy.music.play_BANG_ = (function joy$music$play_BANG_(note_fn,notes){
var temp__4655__auto__ = (function (){var or__4212__auto__ = window.AudioContext;
if(cljs.core.truth_(or__4212__auto__)){
return or__4212__auto__;
} else {
return window.webkitAudioContext;
}
})();
if(cljs.core.truth_(temp__4655__auto__)){
var ctor = temp__4655__auto__;
var ctx = (joy.music.make_once.cljs$core$IFn$_invoke$arity$1 ? joy.music.make_once.cljs$core$IFn$_invoke$arity$1(ctor) : joy.music.make_once.call(null,ctor));
var compressor = ctx.createDynamicsCompressor();
var now_6043 = ctx.currentTime;
var seq__6031_6044 = cljs.core.seq(notes);
var chunk__6032_6045 = null;
var count__6033_6046 = (0);
var i__6034_6047 = (0);
while(true){
if((i__6034_6047 < count__6033_6046)){
var note_6048 = chunk__6032_6045.cljs$core$IIndexed$_nth$arity$2(null,i__6034_6047);
joy.music.connect_to((function (){var G__6039 = ctx;
var G__6040 = cljs.core.update_in.cljs$core$IFn$_invoke$arity$4(note_6048,new cljs.core.PersistentVector(null, 1, 5, cljs.core.PersistentVector.EMPTY_NODE, [cljs.core.cst$kw$delay], null),cljs.core._PLUS_,now_6043);
return (note_fn.cljs$core$IFn$_invoke$arity$2 ? note_fn.cljs$core$IFn$_invoke$arity$2(G__6039,G__6040) : note_fn.call(null,G__6039,G__6040));
})(),compressor);


var G__6049 = seq__6031_6044;
var G__6050 = chunk__6032_6045;
var G__6051 = count__6033_6046;
var G__6052 = (i__6034_6047 + (1));
seq__6031_6044 = G__6049;
chunk__6032_6045 = G__6050;
count__6033_6046 = G__6051;
i__6034_6047 = G__6052;
continue;
} else {
var temp__4657__auto___6053 = cljs.core.seq(seq__6031_6044);
if(temp__4657__auto___6053){
var seq__6031_6054__$1 = temp__4657__auto___6053;
if(cljs.core.chunked_seq_QMARK_(seq__6031_6054__$1)){
var c__4638__auto___6055 = cljs.core.chunk_first(seq__6031_6054__$1);
var G__6056 = cljs.core.chunk_rest(seq__6031_6054__$1);
var G__6057 = c__4638__auto___6055;
var G__6058 = cljs.core.count(c__4638__auto___6055);
var G__6059 = (0);
seq__6031_6044 = G__6056;
chunk__6032_6045 = G__6057;
count__6033_6046 = G__6058;
i__6034_6047 = G__6059;
continue;
} else {
var note_6060 = cljs.core.first(seq__6031_6054__$1);
joy.music.connect_to((function (){var G__6041 = ctx;
var G__6042 = cljs.core.update_in.cljs$core$IFn$_invoke$arity$4(note_6060,new cljs.core.PersistentVector(null, 1, 5, cljs.core.PersistentVector.EMPTY_NODE, [cljs.core.cst$kw$delay], null),cljs.core._PLUS_,now_6043);
return (note_fn.cljs$core$IFn$_invoke$arity$2 ? note_fn.cljs$core$IFn$_invoke$arity$2(G__6041,G__6042) : note_fn.call(null,G__6041,G__6042));
})(),compressor);


var G__6061 = cljs.core.next(seq__6031_6054__$1);
var G__6062 = null;
var G__6063 = (0);
var G__6064 = (0);
seq__6031_6044 = G__6061;
chunk__6032_6045 = G__6062;
count__6033_6046 = G__6063;
i__6034_6047 = G__6064;
continue;
}
} else {
}
}
break;
}

return joy.music.connect_to(compressor,ctx.destination);
} else {
return alert("Sorry, this browser doesn't support AudioContext");
}
});
joy.music.play_BANG_(joy.music.woo,new cljs.core.PersistentVector(null, 1, 5, cljs.core.PersistentVector.EMPTY_NODE, [new cljs.core.PersistentArrayMap(null, 4, [cljs.core.cst$kw$cent,(1100),cljs.core.cst$kw$duration,(1),cljs.core.cst$kw$delay,(0),cljs.core.cst$kw$volume,0.4], null)], null));
