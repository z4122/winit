#![allow(dead_code)]
#![allow(non_snake_case_functions)]
#![allow(non_camel_case_types)]

use libc;

pub type Bool = libc::c_int;
pub type Colormap = XID;
pub type Cursor = XID;
pub type Display = ();
pub type GLXContext = *const ();
pub type GLXContextID = XID;
pub type GLXDrawable = XID;
pub type GLXFBConfig = ();
pub type GLXPbuffer = XID;
pub type GLXPixmap = XID;
pub type GLXWindow = XID;
pub type Pixmap = XID;
pub type Visual = ();   // TODO: not sure
pub type VisualID = libc::c_ulong;   // TODO: not sure
pub type Window = XID;
pub type XID = uint;

pub static AllocNone: libc::c_int = 0;
pub static AllocAll: libc::c_int = 1;

pub static InputOutput: libc::c_uint = 1;
pub static InputOnly: libc::c_uint = 2;

pub static CWBackPixmap: libc::c_ulong = (1<<0);
pub static CWBackPixel: libc::c_ulong = (1<<1);
pub static CWBorderPixmap: libc::c_ulong = (1<<2);
pub static CWBorderPixel: libc::c_ulong = (1<<3);
pub static CWBitGravity: libc::c_ulong = (1<<4);
pub static CWWinGravity: libc::c_ulong = (1<<5);
pub static CWBackingStore: libc::c_ulong = (1<<6);
pub static CWBackingPlanes: libc::c_ulong = (1<<7);
pub static CWBackingPixel: libc::c_ulong = (1<<8);
pub static CWOverrideRedirect: libc::c_ulong = (1<<9);
pub static CWSaveUnder: libc::c_ulong = (1<<10);
pub static CWEventMask: libc::c_ulong = (1<<11);
pub static CWDontPropagate: libc::c_ulong = (1<<12);
pub static CWColormap: libc::c_ulong = (1<<13);
pub static CWCursor: libc::c_ulong = (1<<14);

pub static GLX_USE_GL: libc::c_int = 1;
pub static GLX_BUFFER_SIZE: libc::c_int = 2;
pub static GLX_LEVEL: libc::c_int = 3;
pub static GLX_RGBA: libc::c_int = 4;
pub static GLX_DOUBLEBUFFER: libc::c_int = 5;
pub static GLX_STEREO: libc::c_int = 6;
pub static GLX_AUX_BUFFERS: libc::c_int = 7;
pub static GLX_RED_SIZE: libc::c_int = 8;
pub static GLX_GREEN_SIZE: libc::c_int = 9;
pub static GLX_BLUE_SIZE: libc::c_int = 10;
pub static GLX_ALPHA_SIZE: libc::c_int = 11;
pub static GLX_DEPTH_SIZE: libc::c_int = 12;
pub static GLX_STENCIL_SIZE: libc::c_int = 13;
pub static GLX_ACCUM_RED_SIZE: libc::c_int = 14;
pub static GLX_ACCUM_GREEN_SIZE: libc::c_int = 15;
pub static GLX_ACCUM_BLUE_SIZE: libc::c_int = 16;
pub static GLX_ACCUM_ALPHA_SIZE: libc::c_int = 17;
pub static GLX_BAD_SCREEN: libc::c_int = 1;
pub static GLX_BAD_ATTRIBUTE: libc::c_int = 2;
pub static GLX_NO_EXTENSION: libc::c_int = 3;
pub static GLX_BAD_VISUAL: libc::c_int = 4;
pub static GLX_BAD_CONTEXT: libc::c_int = 5;
pub static GLX_BAD_VALUE: libc::c_int = 6;
pub static GLX_BAD_ENUM: libc::c_int = 7;
pub static GLX_VENDOR: libc::c_int = 1;
pub static GLX_VERSION: libc::c_int = 2;
pub static GLX_EXTENSIONS: libc::c_int = 3;
pub static GLX_WINDOW_BIT: libc::c_int = 0x00000001;
pub static GLX_PIXMAP_BIT: libc::c_int = 0x00000002;
pub static GLX_PBUFFER_BIT: libc::c_int = 0x00000004;
pub static GLX_RGBA_BIT: libc::c_int = 0x00000001;
pub static GLX_COLOR_INDEX_BIT: libc::c_int = 0x00000002;
pub static GLX_PBUFFER_CLOBBER_MASK: libc::c_int = 0x08000000;
pub static GLX_FRONT_LEFT_BUFFER_BIT: libc::c_int = 0x00000001;
pub static GLX_FRONT_RIGHT_BUFFER_BIT: libc::c_int = 0x00000002;
pub static GLX_BACK_LEFT_BUFFER_BIT: libc::c_int = 0x00000004;
pub static GLX_BACK_RIGHT_BUFFER_BIT: libc::c_int = 0x00000008;
pub static GLX_AUX_BUFFERS_BIT: libc::c_int = 0x00000010;
pub static GLX_DEPTH_BUFFER_BIT: libc::c_int = 0x00000020;
pub static GLX_STENCIL_BUFFER_BIT: libc::c_int = 0x00000040;
pub static GLX_ACCUM_BUFFER_BIT: libc::c_int = 0x00000080;
pub static GLX_CONFIG_CAVEAT: libc::c_int = 0x20;
pub static GLX_X_VISUAL_TYPE: libc::c_int = 0x22;
pub static GLX_TRANSPARENT_TYPE: libc::c_int = 0x23;
pub static GLX_TRANSPARENT_INDEX_VALUE: libc::c_int = 0x24;
pub static GLX_TRANSPARENT_RED_VALUE: libc::c_int = 0x25;
pub static GLX_TRANSPARENT_GREEN_VALUE: libc::c_int = 0x26;
pub static GLX_TRANSPARENT_BLUE_VALUE: libc::c_int = 0x27;
pub static GLX_TRANSPARENT_ALPHA_VALUE: libc::c_int = 0x28;
pub static GLX_DONT_CARE: libc::c_int = 0xFFFFFFFF;
pub static GLX_NONE: libc::c_int = 0x8000;
pub static GLX_SLOW_CONFIG: libc::c_int = 0x8001;
pub static GLX_TRUE_COLOR: libc::c_int = 0x8002;
pub static GLX_DIRECT_COLOR: libc::c_int = 0x8003;
pub static GLX_PSEUDO_COLOR: libc::c_int = 0x8004;
pub static GLX_STATIC_COLOR: libc::c_int = 0x8005;
pub static GLX_GRAY_SCALE: libc::c_int = 0x8006;
pub static GLX_STATIC_GRAY: libc::c_int = 0x8007;
pub static GLX_TRANSPARENT_RGB: libc::c_int = 0x8008;
pub static GLX_TRANSPARENT_INDEX: libc::c_int = 0x8009;
pub static GLX_VISUAL_ID: libc::c_int = 0x800B;
pub static GLX_SCREEN: libc::c_int = 0x800C;
pub static GLX_NON_CONFORMANT_CONFIG: libc::c_int = 0x800D;
pub static GLX_DRAWABLE_TYPE: libc::c_int = 0x8010;
pub static GLX_RENDER_TYPE: libc::c_int = 0x8011;
pub static GLX_X_RENDERABLE: libc::c_int = 0x8012;
pub static GLX_FBCONFIG_ID: libc::c_int = 0x8013;
pub static GLX_RGBA_TYPE: libc::c_int = 0x8014;
pub static GLX_COLOR_INDEX_TYPE: libc::c_int = 0x8015;
pub static GLX_MAX_PBUFFER_WIDTH: libc::c_int = 0x8016;
pub static GLX_MAX_PBUFFER_HEIGHT: libc::c_int = 0x8017;
pub static GLX_MAX_PBUFFER_PIXELS: libc::c_int = 0x8018;
pub static GLX_PRESERVED_CONTENTS: libc::c_int = 0x801B;
pub static GLX_LARGEST_PBUFFER: libc::c_int = 0x801C;
pub static GLX_WIDTH: libc::c_int = 0x801D;
pub static GLX_HEIGHT: libc::c_int = 0x801E;
pub static GLX_EVENT_MASK: libc::c_int = 0x801F;
pub static GLX_DAMAGED: libc::c_int = 0x8020;
pub static GLX_SAVED: libc::c_int = 0x8021;
pub static GLX_WINDOW: libc::c_int = 0x8022;
pub static GLX_PBUFFER: libc::c_int = 0x8023;
pub static GLX_PBUFFER_HEIGHT: libc::c_int = 0x8040;
pub static GLX_PBUFFER_WIDTH: libc::c_int = 0x8041;

#[repr(C)]
pub struct XVisualInfo {
    pub visual: *mut Visual,
    pub visualid: VisualID,
    pub screen: libc::c_int,
    pub depth: libc::c_int,
    pub class: libc::c_int,
    pub red_mask: libc::c_ulong,
    pub green_mask: libc::c_ulong,
    pub blue_mask: libc::c_ulong,
    pub colormap_size: libc::c_int,
    pub bits_per_rgb: libc::c_int,
}

#[repr(C)]
pub struct XSetWindowAttributes {
    pub background_pixmap: Pixmap,
    pub background_pixel: libc::c_ulong,
    pub border_pixmap: Pixmap,
    pub border_pixel: libc::c_ulong,
    pub bit_gravity: libc::c_int,
    pub win_gravity: libc::c_int,
    pub backing_store: libc::c_int,
    pub backing_planes: libc::c_ulong,
    pub backing_pixel: libc::c_long,
    pub save_under: Bool,
    pub event_mask: libc::c_long,
    pub do_not_propagate_mask: libc::c_long,
    pub override_redirect: Bool,
    pub colormap: Colormap,
    pub cursor: Cursor,
}

#[link(name = "GL")]
#[link(name = "X11")]
extern "C" {
    pub fn XCloseDisplay(display: *mut Display);
    pub fn XCreateColormap(display: *mut Display, w: Window,
        visual: *mut Visual, alloc: libc::c_int) -> Colormap;
    pub fn XCreateWindow(display: *mut Display, parent: Window, x: libc::c_int,
        y: libc::c_int, width: libc::c_uint, height: libc::c_uint,
        border_width: libc::c_uint, depth: libc::c_int, class: libc::c_uint,
        visual: *mut Visual, valuemask: libc::c_ulong,
        attributes: *mut XSetWindowAttributes) -> Window;
    pub fn XDefaultRootWindow(display: *mut Display) -> Window;
    pub fn XDefaultScreen(display: *mut Display) -> libc::c_int;
    pub fn XFlush(display: *mut Display);
    pub fn XMapWindow(display: *mut Display, w: Window);
    pub fn XOpenDisplay(display_name: *const libc::c_char) -> *mut Display;
    pub fn XStoreName(display: *mut Display, w: Window, window_name: *const libc::c_char);

    pub fn glXCreateContext(dpy: *mut Display, vis: *const XVisualInfo,
        shareList: GLXContext, direct: Bool) -> GLXContext;

    pub fn glXChooseFBConfig(dpy: *mut Display, screen: libc::c_int,
        attrib_list: *const libc::c_int, nelements: *mut libc::c_int);

    pub fn glXChooseVisual(dpy: *mut Display, screen: libc::c_int,
        attribList: *const libc::c_int) -> *const XVisualInfo;

    pub fn glXGetProcAddress(procName: *const libc::c_uchar) -> *const ();

    pub fn glXMakeCurrent(dpy: *mut Display, drawable: GLXDrawable,
        ctx: GLXContext) -> Bool;

    pub fn glXSwapBuffers(dpy: *mut Display, drawable: GLXDrawable);
}

/*
GLXFBConfig *glXGetFBConfigs (Display *dpy, int screen, int *nelements);
int glXGetFBConfigAttrib (Display *dpy, GLXFBConfig config, int attribute, int *value);
XVisualInfo *glXGetVisualFromFBConfig (Display *dpy, GLXFBConfig config);
GLXWindow glXCreateWindow (Display *dpy, GLXFBConfig config, Window win, const int *attrib_list);
void glXDestroyWindow (Display *dpy, GLXWindow win);
GLXPixmap glXCreatePixmap (Display *dpy, GLXFBConfig config, Pixmap pixmap, const int *attrib_list);
void glXDestroyPixmap (Display *dpy, GLXPixmap pixmap);
GLXPbuffer glXCreatePbuffer (Display *dpy, GLXFBConfig config, const int *attrib_list);
void glXDestroyPbuffer (Display *dpy, GLXPbuffer pbuf);
void glXQueryDrawable (Display *dpy, GLXDrawable draw, int attribute, unsigned int *value);
GLXContext glXCreateNewContext (Display *dpy, GLXFBConfig config, int render_type, GLXContext share_list, Bool direct);
Bool glXMakeContextCurrent (Display *dpy, GLXDrawable draw, GLXDrawable read, GLXContext ctx);
GLXDrawable glXGetCurrentReadDrawable (void);
int glXQueryContext (Display *dpy, GLXContext ctx, int attribute, int *value);
void glXSelectEvent (Display *dpy, GLXDrawable draw, unsigned long event_mask);
void glXGetSelectedEvent (Display *dpy, GLXDrawable draw, unsigned long *event_mask);

extern void glXDestroyContext( Display *dpy, GLXContext ctx );


extern void glXCopyContext( Display *dpy, GLXContext src, GLXContext dst,
                unsigned long mask );


extern GLXPixmap glXCreateGLXPixmap( Display *dpy, XVisualInfo *visual,
                     Pixmap pixmap );

extern void glXDestroyGLXPixmap( Display *dpy, GLXPixmap pixmap );

extern Bool glXQueryExtension( Display *dpy, int *errorb, int *event );

extern Bool glXQueryVersion( Display *dpy, int *maj, int *min );

extern Bool glXIsDirect( Display *dpy, GLXContext ctx );

extern int glXGetConfig( Display *dpy, XVisualInfo *visual,
             int attrib, int *value );

extern GLXContext glXGetCurrentContext( void );

extern GLXDrawable glXGetCurrentDrawable( void );

extern void glXWaitGL( void );

extern void glXWaitX( void );

extern void glXUseXFont( Font font, int first, int count, int list );

extern const char *glXQueryExtensionsString( Display *dpy, int screen );

extern const char *glXQueryServerString( Display *dpy, int screen, int name );

extern const char *glXGetClientString( Display *dpy, int name );

extern Display *glXGetCurrentDisplay( void );

*/