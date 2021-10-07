// Compiled by ClojureScript 1.10.879 {:optimizations :whitespace}
goog.provide('joy.music');
goog.require('cljs.core');
/**
 * Return a gain node that goes from silent at time <delay>
 *   up to <volume> in 50 milliseconds, then ramps back down
 *   to silent after <duration>
 */
joy.music.soft_attack = (function joy$music$soft_attack(ctx,p__1786){
var map__1787 = p__1786;
var map__1787__$1 = cljs.core.__destructure_map.call(null,map__1787);
var volume = cljs.core.get.call(null,map__1787__$1,new cljs.core.Keyword(null,"volume","volume",1900330799));
var delay = cljs.core.get.call(null,map__1787__$1,new cljs.core.Keyword(null,"delay","delay",-574225219));
var duration = cljs.core.get.call(null,map__1787__$1,new cljs.core.Keyword(null,"duration","duration",1444101068));
var node = ctx.createGainNode();
var G__1788_1789 = node.gain;
G__1788_1789.linearRampToValueAtTime((0),delay);

G__1788_1789.linearRampToValueAtTime(volume,(delay + 0.05));

G__1788_1789.linearRampToValueAtTime((0),(delay + duration));


return node;
});
/**
 * Return an oscillator that plays starting at
 *   <delay> for <duration> seconds
 */
joy.music.sine_tone = (function joy$music$sine_tone(ctx,p__1790){
var map__1791 = p__1790;
var map__1791__$1 = cljs.core.__destructure_map.call(null,map__1791);
var cent = cljs.core.get.call(null,map__1791__$1,new cljs.core.Keyword(null,"cent","cent",-673959391));
var delay = cljs.core.get.call(null,map__1791__$1,new cljs.core.Keyword(null,"delay","delay",-574225219));
var duration = cljs.core.get.call(null,map__1791__$1,new cljs.core.Keyword(null,"duration","duration",1444101068));
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
var note__$1 = cljs.core.update_in.call(null,note,new cljs.core.PersistentVector(null, 1, 5, cljs.core.PersistentVector.EMPTY_NODE, [new cljs.core.Keyword(null,"duration","duration",1444101068)], null),cljs.core._STAR_,linger);
return joy.music.connect_to.call(null,joy.music.sine_tone.call(null,ctx,note__$1),joy.music.soft_attack.call(null,ctx,note__$1));
});
joy.music.make_once = cljs.core.memoize.call(null,(function (ctor){
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
var ctx = joy.music.make_once.call(null,ctor);
var compressor = ctx.createDynamicsCompressor();
var now_1796 = ctx.currentTime;
var seq__1792_1797 = cljs.core.seq.call(null,notes);
var chunk__1793_1798 = null;
var count__1794_1799 = (0);
var i__1795_1800 = (0);
while(true){
if((i__1795_1800 < count__1794_1799)){
var note_1801 = cljs.core._nth.call(null,chunk__1793_1798,i__1795_1800);
joy.music.connect_to.call(null,note_fn.call(null,ctx,cljs.core.update_in.call(null,note_1801,new cljs.core.PersistentVector(null, 1, 5, cljs.core.PersistentVector.EMPTY_NODE, [new cljs.core.Keyword(null,"delay","delay",-574225219)], null),cljs.core._PLUS_,now_1796)),compressor);


var G__1802 = seq__1792_1797;
var G__1803 = chunk__1793_1798;
var G__1804 = count__1794_1799;
var G__1805 = (i__1795_1800 + (1));
seq__1792_1797 = G__1802;
chunk__1793_1798 = G__1803;
count__1794_1799 = G__1804;
i__1795_1800 = G__1805;
continue;
} else {
var temp__4657__auto___1806 = cljs.core.seq.call(null,seq__1792_1797);
if(temp__4657__auto___1806){
var seq__1792_1807__$1 = temp__4657__auto___1806;
if(cljs.core.chunked_seq_QMARK_.call(null,seq__1792_1807__$1)){
var c__4638__auto___1808 = cljs.core.chunk_first.call(null,seq__1792_1807__$1);
var G__1809 = cljs.core.chunk_rest.call(null,seq__1792_1807__$1);
var G__1810 = c__4638__auto___1808;
var G__1811 = cljs.core.count.call(null,c__4638__auto___1808);
var G__1812 = (0);
seq__1792_1797 = G__1809;
chunk__1793_1798 = G__1810;
count__1794_1799 = G__1811;
i__1795_1800 = G__1812;
continue;
} else {
var note_1813 = cljs.core.first.call(null,seq__1792_1807__$1);
joy.music.connect_to.call(null,note_fn.call(null,ctx,cljs.core.update_in.call(null,note_1813,new cljs.core.PersistentVector(null, 1, 5, cljs.core.PersistentVector.EMPTY_NODE, [new cljs.core.Keyword(null,"delay","delay",-574225219)], null),cljs.core._PLUS_,now_1796)),compressor);


var G__1814 = cljs.core.next.call(null,seq__1792_1807__$1);
var G__1815 = null;
var G__1816 = (0);
var G__1817 = (0);
seq__1792_1797 = G__1814;
chunk__1793_1798 = G__1815;
count__1794_1799 = G__1816;
i__1795_1800 = G__1817;
continue;
}
} else {
}
}
break;
}

return joy.music.connect_to.call(null,compressor,ctx.destination);
} else {
return alert("Sorry, this browser doesn't support AudioContext");
}
});
joy.music.play_BANG_.call(null,joy.music.woo,new cljs.core.PersistentVector(null, 1, 5, cljs.core.PersistentVector.EMPTY_NODE, [new cljs.core.PersistentArrayMap(null, 4, [new cljs.core.Keyword(null,"cent","cent",-673959391),(1100),new cljs.core.Keyword(null,"duration","duration",1444101068),(1),new cljs.core.Keyword(null,"delay","delay",-574225219),(0),new cljs.core.Keyword(null,"volume","volume",1900330799),0.4], null)], null));
