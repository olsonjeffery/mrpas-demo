/*
 * libtcod C samples
 * This code demonstrates various usages of libtcod modules
 * It's in the public domain.
 */

#include <stdlib.h> /* for NULL */
#include <string.h>
#include <stdio.h>
#include "include/libtcod.h"
#include <math.h>

#include <mach/mach_time.h>
#define ORWL_NANO (+1.0E-9)
#define ORWL_GIGA UINT64_C(1000000000)

static double orwl_timebase = 0.0;
static uint64_t orwl_timestart = 0;

struct timespec orwl_gettime(void) {
  // be more careful in a multithreaded environement
  if (!orwl_timestart) {
    mach_timebase_info_data_t tb = { 0 };
    mach_timebase_info(&tb);
    orwl_timebase = tb.numer;
    orwl_timebase /= tb.denom;
    orwl_timestart = mach_absolute_time();
  }
  struct timespec t;
  double diff = (mach_absolute_time() - orwl_timestart) * orwl_timebase;
  t.tv_sec = diff * ORWL_NANO;
  t.tv_nsec = diff - (t.tv_sec * ORWL_GIGA);
  return t;
}

/* ***************************
 * fov sample
 * ***************************/
int main() {
	static char *smap[] = {
		"##############################################",
		"#######################      #################",
		"#####################    #     ###############",
		"######################  ###        ###########",
		"##################      #####             ####",
		"################       ########    ###### ####",
		"###############      #################### ####",
		"################    ######                  ##",
		"########   #######  ######   #     #     #  ##",
		"########   ######      ###                  ##",
		"########                                    ##",
		"####       ######      ###   #     #     #  ##",
		"#### ###   ########## ####                  ##",
		"#### ###   ##########   ###########=##########",
		"#### ##################   #####          #####",
		"#### ###             #### #####          #####",
		"####           #     ####                #####",
		"########       #     #### #####          #####",
		"########       #####      ####################",
		"##############################################",
	};
	static int px=20,py=10; 
	static bool recompute_fov=true;
	static bool torch=false;
	static bool light_walls=true;
	static TCOD_map_t map=NULL;
	static TCOD_color_t dark_wall={0,0,100};
	static TCOD_color_t light_wall={130,110,50};
	static TCOD_color_t dark_ground={50,50,150};
	static TCOD_color_t light_ground={200,180,50};
  static int SAMPLE_SCREEN_WIDTH=46;
  static int SAMPLE_SCREEN_HEIGHT=20;
	int x,y;
  struct timespec start_time;
  struct timespec end_time;
	map = TCOD_map_new(SAMPLE_SCREEN_WIDTH,SAMPLE_SCREEN_HEIGHT);
	for (y=0; y < SAMPLE_SCREEN_HEIGHT; y++ ) {
		for (x=0; x < SAMPLE_SCREEN_WIDTH; x++ ) {
			if ( smap[y][x] == ' ' ) TCOD_map_set_properties(map,x,y,true,true);
			else if ( smap[y][x] == '=' ) TCOD_map_set_properties(map,x,y,true,false);
		}
	}
	if ( recompute_fov ) {
		recompute_fov=false;
    start_time = orwl_gettime();
		TCOD_map_compute_fov(map,px,py, 10, light_walls, (TCOD_fov_algorithm_t)10);
    end_time = orwl_gettime();
	}
	for (y=0; y < SAMPLE_SCREEN_HEIGHT; y++ ) {
		for (x=0; x < SAMPLE_SCREEN_WIDTH; x++ ) {
      if (!TCOD_map_is_in_fov(map, x, y)) {
        printf(".");
      } else if (!TCOD_map_is_transparent(map, x, y)) {
        printf("#");
      } else {
        printf(" ");
      }
		}
    printf("\n");
	}
  int timespan = (end_time.tv_nsec - start_time.tv_nsec);
  printf("Run time: %d nsecs\n", timespan);
  return 0;
}
