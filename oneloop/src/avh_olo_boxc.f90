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


module avh_olo_boxc
   use avh_olo_units
   use avh_olo_prec
   use avh_olo_auxfun
   use avh_olo_qmplx
   implicit none
   private
   public :: boxc

   !integer,parameter :: prm(24)=[xx,xx,xx,xx,xx,xx,xx,xx,xx,xx,xx,xx,xx,xx,xx,xx,xx,xx,xx,xx,xx,xx,xx,xx]
                                 !01,02,03,04,05,06,07,08,09,10,11,12,13,14,15,16,17,18,19,20,21,22,23,24
   !integer,parameter :: prmA(24)=[20,23,12,09,13,16,02,05,18,15,19,22,08,11,24,21,01,04,14,17,06,03,07,10] ! 1423
   integer,parameter :: prmA(24)=[17,07,22,18,08,21,23,13,04,24,14,03,05,19,10,06,20,09,11,01,16,12,02,15] ! 1342
   !integer,parameter :: prmA(24)=[16,12,23,13,09,20,22,18,05,19,15,02,04,24,11,01,21,08,10,06,17,07,03,14] ! 1432
   !integer,parameter :: prmA(24)=[07,17,18,22,21,08,13,23,24,04,03,14,19,05,06,10,09,20,01,11,12,16,15,02] ! 2341
   !integer,parameter :: prmA(24)=[12,16,13,23,20,09,18,22,19,05,02,15,24,04,01,11,08,21,06,10,07,17,14,03] ! 2431
   integer,parameter :: prmB(24)=[05,04,01,06,03,02,11,10,07,12,09,08,17,16,13,18,15,14,23,22,19,24,21,20] ! 3124
   !integer,parameter :: prmB(24)=[04,05,06,01,02,03,10,11,12,07,08,09,16,17,18,13,14,15,22,23,24,19,20,21] ! 3214
   !integer,parameter :: prmB(24)=[24,19,10,11,15,14,06,01,16,17,21,20,12,07,22,23,03,02,18,13,04,05,09,08] ! 4213
   !integer,parameter :: prmB(24)=[19,24,11,10,14,15,01,06,17,16,20,21,07,12,23,22,02,03,13,18,05,04,08,09] ! 4123

   integer,parameter :: prm(4,24)=reshape([&
      1,2,3,4 ,2,1,3,4 ,2,3,1,4 ,3,2,1,4 ,3,1,2,4 ,1,3,2,4 & ! 01 02 03 04 05 06
     ,2,3,4,1 ,3,2,4,1 ,3,4,2,1 ,4,3,2,1 ,4,2,3,1 ,2,4,3,1 & ! 07 08 09 10 11 12
     ,3,4,1,2 ,4,3,1,2 ,4,1,3,2 ,1,4,3,2 ,1,3,4,2 ,3,1,4,2 & ! 13 14 15 16 17 18
     ,4,1,2,3 ,1,4,2,3 ,1,2,4,3 ,2,1,4,3 ,2,4,1,3 ,4,2,1,3 & ! 19 20 21 22 23 24
     ],[4,24])
   integer,parameter :: ord(10,24)=reshape([&
      1,2,3,4 ,1,2,3,4,5,6  ,4,2,3,1 ,6,2,5,4,3,1  ,2,4,3,1 ,6,3,5,1,2,4  ,1,4,3,2 ,4,3,2,1,5,6 &
     ,4,1,3,2 ,4,5,2,6,3,1  ,2,1,3,4 ,1,5,3,6,2,4  ,2,3,4,1 ,2,3,4,1,6,5  ,1,3,4,2 ,5,3,6,1,4,2 &
     ,3,1,4,2 ,5,4,6,2,3,1  ,2,1,4,3 ,1,4,3,2,6,5  ,1,2,4,3 ,1,6,3,5,4,2  ,3,2,4,1 ,2,6,4,5,3,1 &
     ,3,4,1,2 ,3,4,1,2,5,6  ,2,4,1,3 ,6,4,5,2,1,3  ,4,2,1,3 ,6,1,5,3,4,2  ,3,2,1,4 ,2,1,4,3,5,6 &
     ,2,3,1,4 ,2,5,4,6,1,3  ,4,3,1,2 ,3,5,1,6,4,2  ,4,1,2,3 ,4,1,2,3,6,5  ,3,1,2,4 ,5,1,6,3,2,4 &
     ,1,3,2,4 ,5,2,6,4,1,3  ,4,3,2,1 ,3,2,1,4,6,5  ,3,4,2,1 ,3,6,1,5,2,4  ,1,4,2,3 ,4,6,2,5,1,3 &
     ],[10,24])

contains

   subroutine boxc( rslt ,pp_in ,mm_in ,ap_in ,smax )
!*******************************************************************
! Finite 1-loop scalar 4-point function for complex internal masses
! Based on the formulas from
!   Dao Thi Nhung and Le Duc Ninh, arXiv:0902.0325 [hep-ph]
!   G. 't Hooft and M.J.G. Veltman, Nucl.Phys.B153:365-401,1979 
!*******************************************************************
   use avh_olo_box ,only: base,casetable,ll=>permtable
   include 'avh_olo_complex.h90'
     ,intent(out) :: rslt(0:2)
   include 'avh_olo_complex.h90'
     ,intent(in)  :: pp_in(6),mm_in(4)
   include 'avh_olo_real.h90'
     ,intent(in)  :: ap_in(6),smax
   include 'avh_olo_complex.h90'
     :: pp(6),mm(4)
   include 'avh_olo_real.h90'
     :: ap(6),aptmp(4),rem,imm,hh,rm(4)
   include 'avh_olo_complex.h90'
     :: a,b,c,d,e,f,g,h,j,k,dpe,epk,x1,x2,sdnt,o1,j1,e1 &
       ,dek,dpf,def,dpk,abc,bgj,jph,cph
   integer :: icase,jcase,ii,hlp(4),jj
   integer,parameter :: imap(4)=[0,2,4,8]
![CALLINGME  write(*,*) 'MESSAGE from OneLOop boxc: you are calling me'
!
   rslt(0)=0 ;rslt(1)=0 ;rslt(2)=0
!
   hh = neglig(prcpar)*smax
   do ii=1,6
     if (ap_in(ii).ge.hh) then ;ap(ii)=ap_in(ii)
                          else ;ap(ii)=0
     endif
   enddo
!
   do ii=1,4
     if (ap(ii).eq.RZRO) then ;pp(ii)=0
                         else ;pp(ii)=pp_in(ii)
     endif
   enddo
   if (ap(5).eq.RZRO) then
     errorcode = errorcode+1
     if (eunit.ge.0) write(eunit,*) 'ERROR in OneLOop boxc: ' &
       ,' |s| too small, putting it by hand'
     ap(5) = hh
     pp(5) = acmplx(sign(hh,areal(pp_in(5))))
   else
     pp(5) = pp_in(5)
   endif
   if (ap(6).eq.RZRO) then
     errorcode = errorcode+1
     if (eunit.ge.0) write(eunit,*) 'ERROR in OneLOop boxc: ' &
       ,' |t| too small, putting it by hand'
     ap(6) = hh
     pp(6) = acmplx(sign(hh,areal(pp_in(6))))
   else
     pp(6) = pp_in(6)
   endif
!
   do ii=1,4
     rm(ii) = areal(mm_in(ii))
     imm = aimag(mm_in(ii))
     hh = EPSN*abs(rm(ii))
     if (abs(imm).lt.hh) imm = -hh
     mm(ii) = acmplx(rm(ii),imm)
   enddo
!
   icase = 0
   do ii=1,4
     if (ap(ii).gt.RZRO) icase = icase + base(ii)
   enddo
!
   if (icase.lt.15) then
! at least one exernal mass equal zero
     jcase = casetable(icase)
     if (jcase.eq.0.or.jcase.eq.1.or.jcase.eq.5) then
! two opposite masses equal zero
       a = pp(ll(5,icase)) - pp(ll(1,icase))
       c = pp(ll(4,icase)) - pp(ll(5,icase)) - pp(ll(3,icase))
       g = pp(ll(2,icase))
       h = pp(ll(6,icase)) - pp(ll(2,icase)) - pp(ll(3,icase))
       d = (mm(ll(3,icase)) - mm(ll(4,icase))) - pp(ll(3,icase))
       e = (mm(ll(1,icase)) - mm(ll(3,icase))) + pp(ll(3,icase)) - pp(ll(4,icase))
       f = mm(ll(4,icase))
       k = (mm(ll(2,icase)) - mm(ll(3,icase))) - pp(ll(6,icase)) + pp(ll(3,icase))
       dpe = (mm(ll(1,icase)) - mm(ll(4,icase))) - pp(ll(4,icase))
       dpk = (mm(ll(2,icase)) - mm(ll(4,icase))) - pp(ll(6,icase))
       dpf = mm(ll(3,icase)) - pp(ll(3,icase))
       rslt(0) = t13fun( a,c,g,h ,d,e,f,k ,dpe,dpk,dpf )
     else
       a = pp(ll(3,icase))
       b = pp(ll(2,icase))
       c = pp(ll(6,icase)) - pp(ll(2,icase)) - pp(ll(3,icase))
       h = pp(ll(4,icase)) - pp(ll(5,icase)) - pp(ll(6,icase)) + pp(ll(2,icase))
       j = pp(ll(5,icase)) - pp(ll(1,icase)) - pp(ll(2,icase))
       d = (mm(ll(3,icase)) - mm(ll(4,icase))) - pp(ll(3,icase))
       e = (mm(ll(2,icase)) - mm(ll(3,icase))) - pp(ll(6,icase)) + pp(ll(3,icase))
       k = (mm(ll(1,icase)) - mm(ll(2,icase))) + pp(ll(6,icase)) - pp(ll(4,icase))
       f = mm(ll(4,icase))
       cph = pp(ll(4,icase)) - pp(ll(5,icase)) - pp(ll(3,icase))
       dpe = (mm(ll(2,icase)) - mm(ll(4,icase))) - pp(ll(6,icase))
       epk = (mm(ll(1,icase)) - mm(ll(3,icase))) + pp(ll(3,icase)) - pp(ll(4,icase))
       dek = (mm(ll(1,icase)) - mm(ll(4,icase))) - pp(ll(4,icase))
       dpf = mm(ll(3,icase)) - pp(ll(3,icase))
       rslt(0) = tfun( a,b  ,c  ,h,j ,d,e  ,f ,k ,dpe,dpf ) &
               - tfun( a,b+j,cph,h,j ,d,epk,f ,k ,dek,dpf )
     endif
   else
! no extenal mass equal zero
!     if    (areal((pp(5)-pp(1)-pp(2))**2-4*pp(1)*pp(2)).gt.RZRO)then ;icase=0 !12, no permutation
!     elseif(areal((pp(6)-pp(2)-pp(3))**2-4*pp(2)*pp(3)).gt.RZRO)then ;icase=8 !23, 1 cyclic permutation
!     elseif(areal((pp(4)-pp(5)-pp(3))**2-4*pp(5)*pp(3)).gt.RZRO)then ;icase=4 !34, 2 cyclic permutations
!     elseif(areal((pp(4)-pp(1)-pp(6))**2-4*pp(1)*pp(6)).gt.RZRO)then ;icase=2 !41, 3 cyclic permutations
!     else
!       errorcode = errorcode+1
!       if (eunit.ge.0) write(eunit,*) 'ERROR in OneLOop boxc: ' &
!         ,'no positive lambda, returning 0'
!       return
!     endif
     aptmp(1) = areal((pp(5)-pp(1)-pp(2))**2-4*pp(1)*pp(2))
     aptmp(2) = areal((pp(6)-pp(2)-pp(3))**2-4*pp(2)*pp(3))
     aptmp(3) = areal((pp(5)-pp(3)-pp(4))**2-4*pp(3)*pp(4))
     aptmp(4) = areal((pp(6)-pp(4)-pp(1))**2-4*pp(4)*pp(1))
     icase = sort_4(aptmp)
     !if (all(aptmp.ge.rZRO)) then ! doesn't work with mpfun90
     if ((aptmp(1).ge.rZRO).and.(aptmp(2).ge.rZRO).and.(aptmp(3).ge.rZRO).and.(aptmp(4).ge.rZRO)) then
       icase = prmB(icase)
     !elseif (all(aptmp.le.rZRO)) then ! doesn't work with mpfun90
     elseif ((aptmp(1).le.rZRO).and.(aptmp(2).le.rZRO).and.(aptmp(3).le.rZRO).and.(aptmp(4).le.rZRO)) then
       errorcode = errorcode+1
       if (eunit.ge.0) write(eunit,*) 'ERROR in OneLOop boxc: ' &
         ,'no positive lambda, returning 0'
       return
     else
       icase = prmA(icase)
     endif
     a = pp(ord(7,icase))
     b = pp(ord(6,icase))
     g = pp(ord(5,icase))
     c = pp(ord(10,icase)) - pp(ord(6,icase)) - pp(ord(7,icase))
     h = pp(ord(8,icase)) - pp(ord(9,icase)) - pp(ord(10,icase)) + pp(ord(6,icase))
     j = pp(ord(9,icase)) - pp(ord(5,icase)) - pp(ord(6,icase))
     d = (mm(ord(3,icase)) - mm(ord(4,icase))) - pp(ord(7,icase))
     e = (mm(ord(2,icase)) - mm(ord(3,icase))) - pp(ord(10,icase)) + pp(ord(7,icase))
     k = (mm(ord(1,icase)) - mm(ord(2,icase))) + pp(ord(10,icase)) - pp(ord(8,icase))
     f = mm(ord(4,icase))
     abc = pp(ord(10,icase))
     bgj = pp(ord(9,icase))
     jph = pp(ord(8,icase)) - pp(ord(5,icase)) - pp(ord(10,icase))
     cph = pp(ord(8,icase)) - pp(ord(9,icase)) - pp(ord(7,icase))
     dpe = (mm(ord(2,icase)) - mm(ord(4,icase))) - pp(ord(10,icase))
     epk = (mm(ord(1,icase)) - mm(ord(3,icase))) + pp(ord(7,icase)) - pp(ord(8,icase))
     dek = (mm(ord(1,icase)) - mm(ord(4,icase))) - pp(ord(8,icase))
     dpf = mm(ord(3,icase)) - pp(ord(7,icase))
     def = mm(ord(2,icase)) - pp(ord(10,icase))
     call solabc( x1,x2 ,sdnt ,g,j,b ,0 )
     if (aimag(sdnt).ne.RZRO) then
       errorcode = errorcode+1
       if (eunit.ge.0) write(eunit,*) 'ERROR in OneLOop boxc: ' &
         ,'no real solution for alpha, returning 0'
       return
     endif
!BAD        if (abs(areal(x1)).gt.abs(areal(x2))) then
     if (abs(areal(x1)).lt.abs(areal(x2))) then !BETTER
       sdnt = x1
       x1 = x2
       x2 = sdnt
     endif
     o1 = 1-x1
     j1 = j+2*g*x1
     e1 = e+k*x1
     rslt(0) =   -tfun( abc,g  ,jph,c+2*b+(h+j)*x1, j1   ,dpe,k  ,f,e1 ,dek,def ) &
             + o1*tfun( a  ,bgj,cph,c+h*x1        , o1*j1,d  ,epk,f,e1 ,dek,dpf ) &
             + x1*tfun( a  ,b  ,c  ,c+h*x1        ,-j1*x1,d  ,e  ,f,e1 ,dpe,dpf )
   endif
   end subroutine


   function t13fun( aa,cc,gg,hh ,dd,ee,ff,jj ,dpe,dpj,dpf ) result(rslt)
!*******************************************************************
! /1   /x                             y
! | dx |  dy -----------------------------------------------------
! /0   /0    (gy^2 + hxy + dx + jy + f)*(ax^2 + cxy + dx + ey + f)
!
! jj should have negative imaginary part
!*******************************************************************
   include 'avh_olo_complex.h90'
     ,intent(in) :: aa,cc,gg,hh ,dd,ee,ff,jj ,dpe,dpj,dpf
   include 'avh_olo_complex.h90'
     :: rslt ,kk,ll,nn,y1,y2,sdnt
!
![CALLINGME  write(*,*) 'MESSAGE from OneLOop t13fun: you are calling me'
!
   kk = hh*aa - cc*gg
   ll = aa*dd + hh*ee - dd*gg - cc*jj
   nn = dd*(ee - jj) + (hh - cc)*(ff-IEPS*abs(areal(ff)))
   call solabc( y1,y2 ,sdnt ,kk,ll,nn ,0 )
!
   rslt = - s3fun( y1,y2 ,CZRO,CONE ,aa   ,ee+cc,dpf ) &
          + s3fun( y1,y2 ,CZRO,CONE ,gg   ,jj+hh,dpf ) &
          - s3fun( y1,y2 ,CZRO,CONE ,gg+hh,dpj  ,ff  ) &
          + s3fun( y1,y2 ,CZRO,CONE ,aa+cc,dpe  ,ff  )
!
   rslt = rslt/kk
   end function


   function t1fun( aa,cc,gg,hh ,dd,ee,ff,jj ,dpe ) result(rslt)
!*******************************************************************
! /1   /x                         1
! | dx |  dy ----------------------------------------------
! /0   /0    (g*x + h*x + j)*(a*x^2 + c*xy + d*x + e*y + f)
!
! jj should have negative imaginary part
!*******************************************************************
   include 'avh_olo_complex.h90'
     ,intent(in) :: aa,cc,gg,hh ,dd,ee,ff,jj,dpe
   include 'avh_olo_complex.h90'
     ::rslt ,kk,ll,nn,y1,y2,sdnt
!
![CALLINGME  write(*,*) 'MESSAGE from OneLOop t1fun: you are calling me'
!
   kk = hh*aa - cc*gg
   ll = hh*dd - cc*jj - ee*gg
   nn = hh*(ff-IEPS*abs(areal(ff))) - ee*jj
   call solabc( y1,y2 ,sdnt ,kk,ll,nn ,0 )
!
   rslt = - s3fun( y1,y2 ,CZRO,CONE ,aa+cc,dpe  ,ff ) &
          + s3fun( y1,y2 ,CZRO,CONE ,CZRO ,gg+hh,jj ) &
          - s3fun( y1,y2 ,CZRO,CONE ,CZRO ,gg   ,jj ) &
          + s3fun( y1,y2 ,CZRO,CONE ,aa   ,dd   ,ff )
!
   rslt = rslt/kk
   end function


   function tfun( aa,bb,cc ,gin,hin ,dd,ee,ff ,jin ,dpe ,dpf ) result(rslt)
!*******************************************************************
! /1   /x                             1
! | dx |  dy ------------------------------------------------------
! /0   /0    (g*x + h*x + j)*(a*x^2 + b*y^2 + c*xy + d*x + e*y + f)
!*******************************************************************
   include 'avh_olo_complex.h90'
     ,intent(in) :: aa,bb,cc ,gin,hin ,dd,ee,ff ,jin ,dpe ,dpf
   include 'avh_olo_complex.h90'
     :: rslt ,gg,hh,jj,zz(2),beta,tmpa(2),tmpb(2) &
       ,tmpc(2),kiz(2),ll,nn,kk,y1,y2,yy(2,2),sdnt
   include 'avh_olo_real.h90'
     :: ab1,ab2,ac1,ac2,abab,acac,abac,det,ap1,ap2 &
                  ,apab,apac,x1(2,2),x2(2,2),xmin
   integer :: iz,iy,izmin,sj
   logical :: pp(2,2),p1,p2
!
![CALLINGME  write(*,*) 'MESSAGE from OneLOop tfun: you are calling me'
!
   sj = sgnIm(jin,-1)
   gg = -sj*gin
   hh = -sj*hin
   jj = -sj*jin
!
   if     (bb.eq.CZRO) then
     rslt = -sj*t1fun( aa,cc,gg,hh ,dd,ee,ff,jj ,dpe )
     return
   elseif (aa.eq.CZRO) then
     rslt = -sj*t1fun( bb+cc,-cc,-gg-hh,gg, -dpe-2*(bb+cc),dd+cc &
                      ,dpe+bb+cc+ff,gg+hh+jj ,-ee-2*bb-cc )
     return
   endif
!
   call solabc( zz(1),zz(2) ,sdnt ,bb,cc,aa ,0 )
   if (abs(zz(1)).gt.abs(zz(2))) then
     beta = zz(1)
     zz(1) = zz(2)
     zz(2) = beta
   endif
!
   do iz=1,2
     beta = zz(iz)
     tmpa(iz) = gg + beta*hh
     tmpb(iz) = cc + 2*beta*bb
     tmpc(iz) = dd + beta*ee
     kiz(iz) =        bb*tmpa(iz)               - hh*tmpb(iz)
     ll      =        ee*tmpa(iz) - hh*tmpc(iz) - jj*tmpb(iz)
     nn      = (ff-IEPS*abs(areal(ff)))*tmpa(iz) - jj*tmpc(iz)
     call solabc( yy(iz,1),yy(iz,2) ,sdnt ,kiz(iz),ll,nn ,0 )
     if (abs(aimag(beta)).ne.RZRO) then
       ab1 = areal(-beta)
       ab2 = aimag(-beta)
       ac1 = ab1+1 !areal(1-beta)
       ac2 = ab2   !aimag(1-beta)
       abab = ab1*ab1 + ab2*ab2
       acac = ac1*ac1 + ac2*ac2
       abac = ab1*ac1 + ab2*ac2
       det = abab*acac - abac*abac
       do iy=1,2
         ap1 = areal(yy(iz,iy))
         ap2 = aimag(yy(iz,iy))
         apab = ap1*ab1 + ap2*ab2
         apac = ap1*ac1 + ap2*ac2
         x1(iz,iy) = ( acac*apab - abac*apac )/det
         x2(iz,iy) = (-abac*apab + abab*apac )/det
       enddo
     else
       do iy=1,2
         x1(iz,iy) = -1
         x2(iz,iy) = -1
       enddo
     endif
   enddo
   xmin = 1
   izmin = 2
   do iz=1,2
   do iy=1,2
     if ( x1(iz,iy).ge.RZRO.and.x2(iz,iy).ge.RZRO &
                 .and.x1(iz,iy)+x2(iz,iy).le.RONE ) then
       pp(iz,iy) = .true.
       if (x1(iz,iy).lt.xmin) then
         xmin = x1(iz,iy)
         izmin = iz
       endif
       if (x2(iz,iy).lt.xmin) then
         xmin = x2(iz,iy)
         izmin = iz
       endif
     else
       pp(iz,iy) = .false.
     endif
   enddo
   enddo
   iz = izmin+1
   if (iz.eq.3) iz = 1
!
   beta = zz(iz)
   kk = kiz(iz)
   y1 = yy(iz,1)
   y2 = yy(iz,2)
   p1 = pp(iz,1)
   p2 = pp(iz,2)
!
   rslt =   s3fun( y1,y2 ,beta ,CONE      ,CZRO    ,hh   ,gg+jj  ) &
          - s3fun( y1,y2 ,CZRO ,CONE-beta ,CZRO    ,gg+hh,   jj  ) &
          + s3fun( y1,y2 ,CZRO ,    -beta ,CZRO    ,gg   ,   jj  ) &
          - s3fun( y1,y2 ,beta ,CONE      ,bb      ,cc+ee,aa+dpf ) &
          + s3fun( y1,y2 ,CZRO ,CONE-beta ,aa+bb+cc,dpe  ,ff     ) &
          - s3fun( y1,y2 ,CZRO ,    -beta ,aa      ,dd   ,ff     )
!
   sdnt = plnr( y1,y2 ,p1,p2, tmpa(iz),tmpb(iz),tmpc(iz) )
   if (aimag(beta).le.RZRO) then ;rslt = rslt + sdnt
                            else ;rslt = rslt - sdnt
   endif
!
   rslt = -sj*rslt/kk
   end function


   function s3fun( y1i,y2i ,dd,ee ,aa,bb,cin ) result(rslt)
!*******************************************************************
! Calculate
!            ( S3(y1i) - S3(y2i) )/( y1i - y2i )
! where
!               /1    ee * ln( aa*x^2 + bb*x + cc )
!       S3(y) = |  dx -----------------------------
!               /0           ee*x - y - dd
!
! y1i,y2i should have a non-zero imaginary part
!*******************************************************************
   include 'avh_olo_complex.h90'
     ,intent(in) ::  y1i,y2i ,dd,ee ,aa,bb,cin
   include 'avh_olo_complex.h90'
     :: rslt ,y1,y2,fy1y2,z1,z2,tmp,cc
   include 'avh_olo_real.h90'
     ::rea,reb,rez1,rez2,imz1,imz2,simc,hh
!
![CALLINGME  write(*,*) 'MESSAGE from OneLOop s3fun: you are calling me'
!
   if (ee.eq.CZRO) then
     rslt = 0
     return
   endif
!
   cc = cin
   rea = abs(aa)
   reb = abs(bb)
   simc = abs(cc)
   if (simc.lt.8*neglig(prcpar)*min(rea,reb)) cc = 0
!
   simc = aimag(cc)
   if (simc.eq.RZRO) then
     simc = aimag(bb)
     if (simc.eq.RZRO) simc = -1
   endif
   simc = sgnRe(simc)
!
   y1 = (dd+y1i)/ee
   y2 = (dd+y2i)/ee
   if (aimag(y1).eq.RZRO) then
     errorcode = errorcode+1
     if (eunit.ge.0) write(eunit,*) 'ERROR in OneLOop s3fun: ' &
       ,'y1 has zero imaginary part'
   endif
   if (aimag(y2).eq.RZRO) then
     errorcode = errorcode+1
     if (eunit.ge.0) write(eunit,*) 'ERROR in OneLOop s3fun: ' &
       ,'y2 has zero imaginary part'
   endif
   fy1y2 = r0fun( y1,y2 )
!
   if     (aa.ne.CZRO) then
!
!     call solabc( z1,z2 ,tmp ,aa,bb,cc ,0 )
     call solabc_rcc( z1,z2 ,areal(aa),bb,cc )
     rea  = sgnRe(aa)
     rez1 = areal(z1)
     rez2 = areal(z2) 
     imz1 = aimag(z1) ! sign(Im(a*z1*z2)) = simc
     imz2 = aimag(z2)
     hh = abs(EPSN2*rez1)
!     if (abs(imz1).lt.EPSN*hh) imz1 = simc*rea*sgnRe(rez2)*hh
     if (imz1.eq.RZRO) imz1 = simc*rea*sgnRe(rez2)*hh
     hh = abs(EPSN2*rez2)
!     if (abs(imz2).lt.EPSN*hh) imz2 = simc*rea*sgnRe(rez1)*hh
     if (imz2.eq.RZRO) imz2 = simc*rea*sgnRe(rez1)*hh
     z1 = acmplx( rez1,imz1)
     z2 = acmplx( rez2,imz2)
     rslt = fy1y2 * ( logc(qonv(aa,simc)) &
                    + eta3( -z1,-imz1,-z2,-imz2,CZRO,simc*rea ) ) &
          + r1fun( z1,y1,y2,fy1y2 ) &
          + r1fun( z2,y1,y2,fy1y2 )
!
   elseif (bb.ne.CZRO) then
!
     z1 = -cc/bb ! - i|eps|Re(b)
     reb  = areal(bb)
     rez1 = areal(z1)
     imz1 = aimag(z1)
     if (abs(imz1).eq.RZRO) then
       imz1 = -simc*reb*abs(EPSN2*rez1/reb)
       z1 = acmplx( rez1,imz1)
     endif
     rslt = fy1y2 * ( logc(qonv(bb,simc)) &
                    + eta3(bb,simc ,-z1,-imz1 ,cc,simc) ) &
          + r1fun( z1,y1,y2,fy1y2 )
!
   elseif (cc.ne.CZRO) then
!
     rslt = logc( qonv(cc,simc) )*fy1y2
!
   else!if (aa=bb=cc=0)
!
     errorcode = errorcode+1
     if (eunit.ge.0) write(eunit,*) 'ERROR in OneLOop s3fun: ' &
       ,'cc equal zero, returning 0'
     rslt = 0
!
   endif
!
   rslt = rslt/ee
   end function


   function r1fun( zz,y1,y2,fy1y2 ) result(rslt)
!*******************************************************************
! calculates  ( R1(y1,z) - R1(y2,z) )/( y1 - y2 )
! where
!                          /     / 1-y \       / 1-z \ \
!      R1(y,z) = ln(y-z) * | log |-----| - log |-----| |
!                          \     \ -y  /       \ -z  / / 
!
!                      /    y-z \       /    y-z \
!                - Li2 |1 - ----| + Li2 |1 - ----|
!                      \    -z  /       \    1-z /
!
!                                     / 1-y1 \       / 1-y2 \
!                                 log |------| - log |------| 
! input fy1y2 should be equal to      \  -y1 /       \  -y2 /
!                                 ---------------------------
!                                           y1 - y2
!*******************************************************************
   include 'avh_olo_complex.h90'
     ,intent(in) :: y1,y2,zz,fy1y2
   include 'avh_olo_complex.h90'
     :: rslt ,oz
   type(qmplx_type) :: q1z,q2z,qq
   include 'avh_olo_real.h90'
     :: h12,hz1,hz2,hzz,hoz
   logical :: zzsmall,ozsmall
!
![CALLINGME  write(*,*) 'MESSAGE from OneLOop r1fun: you are calling me'
!
   oz = 1-zz
   h12 = abs(y1-y2)
   hz1 = abs(y1-zz)
   hz2 = abs(y2-zz)
   hzz = abs(zz)
   hoz = abs(oz)
   q1z = qonv(y1-zz)
   q2z = qonv(y2-zz)
!
   zzsmall = .false.
   ozsmall = .false.
   if     (hzz.lt.hz1.and.hzz.lt.hz2.and.hzz.lt.hoz) then ! |z| < |y1-z|,|y2-z|
     zzsmall = .true.
     rslt = fy1y2*logc( q1z ) &
          - ( logc(q1z*q2z)/2 + logc(qonv((y2-1)/y2)) &
                                     - logc(qonv(oz)) )*logc2(q1z/q2z)/(y2-zz)
   elseif (hoz.lt.hz1.and.hoz.lt.hz2) then ! |1-z| < |y1-z|,|y2-z|
     ozsmall = .true.
     rslt = fy1y2*logc( q1z ) &
          - (-logc(q1z*q2z)/2 + logc(qonv((y2-1)/y2)) &
                                    + logc(qonv(-zz)) )*logc2(q1z/q2z)/(y2-zz)
   elseif (h12.le.hz2.and.hz2.le.hz1) then ! |y1-y2| < |y2-z| < |y1-z|
     rslt = fy1y2*logc( q1z ) - r0fun( y2,zz )*logc2( q1z/q2z )        
   elseif (h12.le.hz1.and.hz1.le.hz2) then ! |y1-y2| < |y2-z| < |y1-z|
     rslt = fy1y2*logc( q2z ) - r0fun( y1,zz )*logc2( q2z/q1z )        
   else!if(hz1.lt.h12.or.hz2.lt.h12) then ! |y2-z|,|y1-z| < |y1-y2|
     rslt = 0
     if (hz1.ne.RZRO) rslt = rslt + (y1-zz)*logc( q1z )*r0fun( y1,zz )
     if (hz2.ne.RZRO) rslt = rslt - (y2-zz)*logc( q2z )*r0fun( y2,zz )
     rslt = rslt/(y1-y2)
   endif
!
   if (zzsmall) then ! |z| < |y1-z|,|y2-z|
     qq  = qonv(-zz)
     rslt = rslt + ( li2c( qq/q1z ) - li2c( qq/q2z ) )/(y1-y2)
   else
     qq  = qonv(-zz)
     rslt = rslt + li2c2( q1z/qq ,q2z/qq )/zz
   endif
!
   if (ozsmall) then ! |1-z| < |y1-z|,|y2-z|
     qq  = qonv(oz)
     rslt = rslt - ( li2c( qq/q1z ) - li2c( qq/q2z ) )/(y1-y2)
   else
     qq = qonv(oz)
     rslt = rslt + li2c2( q1z/qq ,q2z/qq )/oz
   endif
   end function


   function r0fun( y1,y2 ) result(rslt)
!*******************************************************************
!      / 1-y1 \       / 1-y2 \
!  log |------| - log |------| 
!      \  -y1 /       \  -y2 /
!  ---------------------------
!            y1 - y2
!
! y1,y2 should have non-zero imaginary parts
!*******************************************************************
   include 'avh_olo_complex.h90'
     ,intent(in) :: y1,y2
   include 'avh_olo_complex.h90'
     :: rslt ,oy1,oy2
!
![CALLINGME  write(*,*) 'MESSAGE from OneLOop r0fun: you are calling me'
   oy1 = 1-y1
   oy2 = 1-y2
   rslt = logc2( qonv(-y2)/qonv(-y1) )/y1 &
        + logc2( qonv(oy2)/qonv(oy1) )/oy1
   end function


   function plnr( y1,y2 ,p1,p2 ,aa,bb,cc ) result(rslt)
!*******************************************************************
!                   /   a    \          /   a    \
!            p1*log |--------| - p2*log |--------| 
!                   \ b*y1+c /          \ b*y2+c /
! 2*pi*imag* -------------------------------------
!                           y1 - y2
! 
! p1,p2 are logical, to be interpreted as 0,1 in the formula above 
!*******************************************************************
   include 'avh_olo_complex.h90'
     ,intent(in) :: y1,y2 ,aa,bb,cc
   logical         ,intent(in) :: p1,p2
   include 'avh_olo_complex.h90'
     :: rslt ,x1,x2,xx
   type(qmplx_type) :: q1,q2
!
   if (p1) then
     x1 = bb*y1 + cc
     xx = aa/x1
     if (aimag(xx).eq.RZRO) then
       errorcode = errorcode+1
       if (eunit.ge.0) write(eunit,*) 'ERROR in OneLOop plnr: ' &
         ,'aa/x1 has zero imaginary part'
     endif
     q1 = qonv(xx)
   endif
   if (p2) then
     x2 = bb*y2 + cc
     xx = aa/x2
     if (aimag(xx).eq.RZRO) then
       errorcode = errorcode+1
       if (eunit.ge.0) write(eunit,*) 'ERROR in OneLOop plnr: ' &
         ,'aa/x2 has zero imaginary part'
     endif
     q2 = qonv(xx)
   endif
   if (p1) then
     if (p2) then
       rslt = logc2( q2/q1 ) * 2*IPI*bb/x2
     else
       rslt = logc( q1 ) * 2*IPI/(y1-y2)
     endif
   elseif (p2) then
     rslt = logc( q2 ) * 2*IPI/(y2-y1) ! minus sign
   else
     rslt = 0
   endif
   end function


  function sort_4(i) result(rslt)
!***********************************************************************
! Hardwired decision tree following insertion sort.
! Maximum number of comparisons is not optimal (6 instead of 5),
! but average number of comparisions is less. 
!***********************************************************************
  include 'avh_olo_real.h90'
    ,intent(in) :: i(4)
  integer,parameter :: outp(24)=&
    [10,09,08,04,11,12,07,03,24,23,22,02,14,13,18,05,15,16,17,06,19,20,21,01] ! large2small
    ![01,21,20,19,06,17,16,15,05,18,13,14,02,22,23,24,03,07,12,11,04,08,09,10] !small2large
    ![01,21,17,07,06,20,16,12,03,23,13,09,02,22,18,08,05,19,15,11,04,24,14,10] !permutation
  integer :: rslt
  if(i(1).le.i(2))then;if(i(2).le.i(3))then;if(i(3).le.i(4))then
    rslt = outp(01)
  elseif(i(2).le.i(4))then
    rslt = outp(02)
  elseif(i(1).le.i(4))then
    rslt = outp(03)
  else
    rslt = outp(04)
  endif;elseif(i(1).le.i(3))then;if(i(2).le.i(4))then
    rslt = outp(05)
  elseif(i(3).le.i(4))then
    rslt = outp(06)
  elseif(i(1).le.i(4))then
    rslt = outp(07)
  else
    rslt = outp(08)
  endif;else;if(i(2).le.i(4))then
    rslt = outp(09)
  elseif(i(1).le.i(4))then
    rslt = outp(10)
  elseif(i(3).le.i(4))then
    rslt = outp(11)
  else
    rslt = outp(12)
  endif;endif;else;if(i(1).le.i(3))then;if(i(3).le.i(4))then
    rslt = outp(13)
  elseif(i(1).le.i(4))then
    rslt = outp(14)
  elseif(i(2).le.i(4))then
    rslt = outp(15)
  else
    rslt = outp(16)
  endif;elseif(i(2).le.i(3))then;if(i(1).le.i(4))then
    rslt = outp(17)
  elseif(i(3).le.i(4))then
    rslt = outp(18)
  elseif(i(2).le.i(4))then
    rslt = outp(19)
  else
    rslt = outp(20)
  endif;else;if(i(1).le.i(4))then
    rslt = outp(21)
  elseif(i(2).le.i(4))then
    rslt = outp(22)
  elseif(i(3).le.i(4))then
    rslt = outp(23)
  else
    rslt = outp(24)
  endif;endif;endif
  end function


end module
