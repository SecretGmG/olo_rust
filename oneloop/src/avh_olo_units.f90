!!
!! Copyright (C) 2018 Andreas van Hameren. 
!!
!! This file is part of OneLOop-rolln.
!!
!! OneLOop-rolln is free software: you can redistribute it and/or modify
!! it under the terms of the GNU General Public License as published by
!! the Free Software Foundation, either version 3 of the License, or
!! (at your option) any later version.
!!
!! OneLOop-rolln is distributed in the hope that it will be useful,
!! but WITHOUT ANY WARRANTY; without even the implied warranty of
!! MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
!! GNU General Public License for more details.
!!
!! You should have received a copy of the GNU General Public License
!! along with OneLOop-rolln.  If not, see <http://www.gnu.org/licenses/>.
!!


module avh_olo_units
  implicit none
  integer :: eunit=6
  integer :: wunit=6
  integer :: munit=6
  integer :: punit=-1 ! print all
  integer :: errorcode=0
  integer :: error_dilog_r=1
  integer :: error_dilog2_r(5)=1
  protected :: eunit,wunit,munit,punit !]PROTECTED
  protected :: error_dilog_r !]PROTECTED
contains
  subroutine set_unit( message ,val )
!***********************************************************************
! message is intended to be one of the following:
! 'printall', 'message' ,'warning' ,'error'
!***********************************************************************
  character(*) ,intent(in) :: message
  integer      ,intent(in) :: val
  select case (trim(message))
  case('printall') ;punit=val
  case('message' ) ;munit=val
  case('warning' ) ;wunit=val
  case('error'   ) ;eunit=val
  case default 
    eunit=val
    wunit=val
    munit=val
    punit=-1
  end select
  end subroutine

  subroutine switch_it( switchname )
!***********************************************************************
!***********************************************************************
  character(*) ,intent(in) :: switchname
  select case (trim(switchname))
  case('error_dilog_r-off') ;error_dilog_r=0 !stops printing this error message
  case('error_dilog_r-on' ) ;error_dilog_r=1 !does print this error message
  case('error_dilog2_r_1-off') ;error_dilog2_r(1)=0
  case('error_dilog2_r_1-on' ) ;error_dilog2_r(1)=1
  case('error_dilog2_r_2-off') ;error_dilog2_r(2)=0
  case('error_dilog2_r_2-on' ) ;error_dilog2_r(2)=1
  case('error_dilog2_r_3-off') ;error_dilog2_r(3)=0
  case('error_dilog2_r_3-on' ) ;error_dilog2_r(3)=1
  case('error_dilog2_r_4-off') ;error_dilog2_r(4)=0
  case('error_dilog2_r_4-on' ) ;error_dilog2_r(4)=1
  case('error_dilog2_r_5-off') ;error_dilog2_r(5)=0
  case('error_dilog2_r_5-on' ) ;error_dilog2_r(5)=1
  end select
  end subroutine
end module
