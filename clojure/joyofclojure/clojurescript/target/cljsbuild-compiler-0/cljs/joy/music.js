// Compiled by ClojureScript 1.10.879 {:optimizations :whitespace}
goog.provide('cljs.joy.music');
goog.require('cljs.core');
/**
 * Return a gain node that goes from silent at time <delay>
 *   up to <volume> in 50 milliseconds, then ramps back down
 *   to silent after <duration>
 */
cljs.joy.music.soft_attack = (function cljs$joy$music$soft_attack(ctx,p__4481){
var map__4482 = p__4481;
var map__4482__$1 = cljs.core.__destructure_map.call(null,map__4482);
var volume = cljs.core.get.call(null,map__4482__$1,new cljs.core.Keyword(null,"volume","volume",1900330799));
var delay = cljs.core.get.call(null,map__4482__$1,new cljs.core.Keyword(null,"delay","delay",-574225219));
var duration = cljs.core.get.call(null,map__4482__$1,new cljs.core.Keyword(null,"duration","duration",1444101068));
var node = ctx.createGainNode();
var G__4483_4484 = node.gain;
G__4483_4484.linearRampToValueAtTime((0),delay);

G__4483_4484.linearRampToValueAtTime(volume,(delay + 0.05));

G__4483_4484.linearRampToValueAtTime((0),(delay + duration));


return node;
});
/**
 * Return an oscillator that plays starting at
 *   <delay> for <duration> seconds
 */
cljs.joy.music.sine_tone = (function cljs$joy$music$sine_tone(ctx,p__4485){
var map__4486 = p__4485;
var map__4486__$1 = cljs.core.__destructure_map.call(null,map__4486);
var cent = cljs.core.get.call(null,map__4486__$1,new cljs.core.Keyword(null,"cent","cent",-673959391));
var delay = cljs.core.get.call(null,map__4486__$1,new cljs.core.Keyword(null,"delay","delay",-574225219));
var duration = cljs.core.get.call(null,map__4486__$1,new cljs.core.Keyword(null,"duration","duration",1444101068));
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
cljs.joy.music.connect_to = (function cljs$joy$music$connect_to(node1,node2){
node1.connect(node2);

return node2;
});
/**
 * Play a 'woo' sound; sounds a bit like a glass harp.
 */
cljs.joy.music.woo = (function cljs$joy$music$woo(ctx,note){
var linger = 1.5;
var note__$1 = cljs.core.update_in.call(null,note,new cljs.core.PersistentVector(null, 1, 5, cljs.core.PersistentVector.EMPTY_NODE, [new cljs.core.Keyword(null,"duration","duration",1444101068)], null),cljs.core._STAR_,linger);
return cljs.joy.music.connect_to.call(null,cljs.joy.music.sine_tone.call(null,ctx,note__$1),cljs.joy.music.soft_attack.call(null,ctx,note__$1));
});
cljs.joy.music.make_once = cljs.core.memoize.call(null,(function (ctor){
return (new ctor());
}));
/**
 * Kick off playing a sequence of notes. note-fn must take
 *   two arguments, an AudioContext object and a map
 *   representing one note to play. It must return an AudioNode
 *   object that will play that note.
 */
cljs.joy.music.play_BANG_ = (function cljs$joy$music$play_BANG_(note_fn,notes){
var temp__4655__auto__ = (function (){var or__4212__auto__ = window.AudioContext;
if(cljs.core.truth_(or__4212__auto__)){
return or__4212__auto__;
} else {
return window.webkitAudioContext;
}
})();
if(cljs.core.truth_(temp__4655__auto__)){
var ctor = temp__4655__auto__;
var ctx = cljs.joy.music.make_once.call(null,ctor);
var compressor = ctx.createDynamicsCompressor();
var now_4491 = ctx.currentTime;
var seq__4487_4492 = cljs.core.seq.call(null,notes);
var chunk__4488_4493 = null;
var count__4489_4494 = (0);
var i__4490_4495 = (0);
while(true){
if((i__4490_4495 < count__4489_4494)){
var note_4496 = cljs.core._nth.call(null,chunk__4488_4493,i__4490_4495);
cljs.joy.music.connect_to.call(null,note_fn.call(null,ctx,cljs.core.update_in.call(null,note_4496,new cljs.core.PersistentVector(null, 1, 5, cljs.core.PersistentVector.EMPTY_NODE, [new cljs.core.Keyword(null,"delay","delay",-574225219)], null),cljs.core._PLUS_,now_4491)),compressor);


var G__4497 = seq__4487_4492;
var G__4498 = chunk__4488_4493;
var G__4499 = count__4489_4494;
var G__4500 = (i__4490_4495 + (1));
seq__4487_4492 = G__4497;
chunk__4488_4493 = G__4498;
count__4489_4494 = G__4499;
i__4490_4495 = G__4500;
continue;
} else {
var temp__4657__auto___4501 = cljs.core.seq.call(null,seq__4487_4492);
if(temp__4657__auto___4501){
var seq__4487_4502__$1 = temp__4657__auto___4501;
if(cljs.core.chunked_seq_QMARK_.call(null,seq__4487_4502__$1)){
var c__4638__auto___4503 = cljs.core.chunk_first.call(null,seq__4487_4502__$1);
var G__4504 = cljs.core.chunk_rest.call(null,seq__4487_4502__$1);
var G__4505 = c__4638__auto___4503;
var G__4506 = cljs.core.count.call(null,c__4638__auto___4503);
var G__4507 = (0);
seq__4487_4492 = G__4504;
chunk__4488_4493 = G__4505;
count__4489_4494 = G__4506;
i__4490_4495 = G__4507;
continue;
} else {
var note_4508 = cljs.core.first.call(null,seq__4487_4502__$1);
cljs.joy.music.connect_to.call(null,note_fn.call(null,ctx,cljs.core.update_in.call(null,note_4508,new cljs.core.PersistentVector(null, 1, 5, cljs.core.PersistentVector.EMPTY_NODE, [new cljs.core.Keyword(null,"delay","delay",-574225219)], null),cljs.core._PLUS_,now_4491)),compressor);


var G__4509 = cljs.core.next.call(null,seq__4487_4502__$1);
var G__4510 = null;
var G__4511 = (0);
var G__4512 = (0);
seq__4487_4492 = G__4509;
chunk__4488_4493 = G__4510;
count__4489_4494 = G__4511;
i__4490_4495 = G__4512;
continue;
}
} else {
}
}
break;
}

return cljs.joy.music.connect_to.call(null,compressor,ctx.destination);
} else {
return alert("Sorry, this browser doesn't support AudioContext");
}
});
cljs.joy.music.play_BANG_.call(null,cljs.joy.music.woo,new cljs.core.PersistentVector(null, 1, 5, cljs.core.PersistentVector.EMPTY_NODE, [new cljs.core.PersistentArrayMap(null, 4, [new cljs.core.Keyword(null,"cent","cent",-673959391),(1100),new cljs.core.Keyword(null,"duration","duration",1444101068),(1),new cljs.core.Keyword(null,"delay","delay",-574225219),(0),new cljs.core.Keyword(null,"volume","volume",1900330799),0.4], null)], null));
