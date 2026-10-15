use bevy::prelude::Reflect;
use serde::{Deserialize,Serialize};
use trinity_macros::TrinityClass;
use crate::{PixelFormat,TextureType};
#[derive(Clone,Copy,Debug,Eq,PartialEq,Reflect,Serialize,Deserialize,TrinityClass)]
#[trinity(module="trinity",name="Tr2BitmapDimensions",legacy_header="trinityal/Tr2BitmapDimensions.h",kind="value")]
pub struct Tr2BitmapDimensions{#[trinity(read,write,persist)]pub texture_type:TextureType,#[trinity(read,write,persist)]pub format:PixelFormat,#[trinity(read,write,persist)]pub width:u32,#[trinity(read,write,persist)]pub height:u32,#[trinity(read,write,persist)]pub volume_depth:u32,#[trinity(read,write,persist)]pub mip_count:u32,#[trinity(read,write,persist)]pub array_size:u32}
impl Default for Tr2BitmapDimensions{fn default()->Self{Self{texture_type:TextureType::Invalid,format:PixelFormat::Unknown,width:0,height:0,volume_depth:0,mip_count:0,array_size:0}}}
impl Tr2BitmapDimensions{
#[must_use]pub const fn get_width(self)->u32{self.width} #[must_use]pub const fn get_height(self)->u32{self.height} #[must_use]pub const fn get_depth(self)->u32{self.volume_depth} #[must_use]pub const fn get_mip_count(self)->u32{self.mip_count} #[must_use]pub const fn get_format(self)->PixelFormat{self.format} #[must_use]pub const fn get_type(self)->TextureType{self.texture_type}
#[must_use]pub const fn get_true_mip_count(self)->u32{if self.mip_count!=0{return self.mip_count;}let extent=match self.texture_type{TextureType::Texture3D=>max3(self.width,self.height,self.volume_depth),TextureType::Texture2D|TextureType::Cube=>max2(self.width,self.height),TextureType::Invalid=>0};ceil_log2(extent)}
#[must_use]pub const fn get_mip_width(self,mip:u32)->u32{mip_extent(self.width,mip,self.get_true_mip_count())} #[must_use]pub const fn get_mip_height(self,mip:u32)->u32{mip_extent(self.height,mip,self.get_true_mip_count())} #[must_use]pub const fn get_mip_depth(self,mip:u32)->u32{if self.texture_type!=TextureType::Texture3D{return 0;}mip_extent(self.volume_depth,mip,self.get_true_mip_count())}
#[must_use]pub const fn get_mip_size(self,mip:u32)->u32{let w=self.get_mip_width(mip);let h=self.get_mip_height(mip);if w==0||h==0{return 0;}w.saturating_mul(h).saturating_mul(self.format.bits_per_pixel())/8}
}
const fn max2(a:u32,b:u32)->u32{if a>b{a}else{b}} const fn max3(a:u32,b:u32,c:u32)->u32{max2(max2(a,b),c)} const fn ceil_log2(v:u32)->u32{if v<=1{return v;}32-(v-1).leading_zeros()} const fn mip_extent(base:u32,mip:u32,count:u32)->u32{if base==0||mip>=count{return 0;}let shifted=base.checked_shr(mip).unwrap_or(0);if shifted==0{1}else{shifted}}
